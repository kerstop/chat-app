use std::collections::HashMap;

use rocket::futures::lock::Mutex;
use rocket::response::stream::{Event, EventStream};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::*;
use rocket::{get, launch, post, routes, Build, Rocket};
use rocket::{Shutdown, State};

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

type Rooms = Mutex<HashMap<String, Sender<String>>>;

#[get("/connect/<room>")]
async fn connect_to_room(rooms: &State<Rooms>, room: &str, mut end: Shutdown) -> EventStream![] {
    let mut lock = rooms.lock().await;
    let mut rx = match lock.get(room) {
        Some(x) => x.subscribe(),
        None => {
            let (tx, rx) = channel(128);
            lock.insert(room.to_string(), tx);
            rx
        }
    };
    drop(lock);

    EventStream! {
        loop {
            select! {
                msg = rx.recv() => {
                    match msg {
                        Ok(m) => yield Event::data(m),
                        Err(_) => break,
                    }
                },
                _ = &mut end => break,
            };
        }
    }
}

#[post("/connect/<room>", data = "<body>")]
async fn send_to_room(room: &str, rooms: &State<Rooms>, body: String) {
    let lock = rooms.lock().await;
    match lock.get(room) {
        Some(s) => {
            match s.send(body) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error sending message: {e}\n\nThe message was: {}", e.0);
                }
            };
        }
        None => (),
    }
}

#[launch]
fn launch() -> Rocket<Build> {
    rocket::build()
    .manage(Mutex::new(HashMap::<String, Sender<String>>::new()))
    .mount("/", routes![hello, connect_to_room, send_to_room])
}
