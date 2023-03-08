mod rooms;

use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::web::Json;
use actix_web::App;
use actix_web::HttpMessage;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::{
    get, post,
    web::{Data, Path},
    Error,
};
use actix_web_actors::ws;
use log::info;
use rooms::Rooms;

#[get("/")]
async fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/connect/{room}")]
async fn connect_to_room(
    rooms: Data<Rooms>,
    room: Path<String>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    rooms.subscribe(room.as_str(), &req, stream).await
}

#[post("/connect/{room}")]
async fn send_to_room(room: Path<String>, rooms: Data<Rooms>, body: Json<String>) -> HttpResponse {
    rooms.send(room.as_str(), body.as_str()).await;

    HttpResponse::new(StatusCode::OK)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    let rooms = Data::new(Rooms::new());

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(connect_to_room)
            .service(send_to_room)
            .app_data(rooms.clone())
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}
