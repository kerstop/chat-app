mod rooms;

use actix_web::HttpMessage;
use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::{
    get, post,
    web::{Data, Path},
};
use actix_web_lab::sse;
use rooms::Rooms;

#[get("/")]
async fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/connect/{room}")]
async fn connect_to_room(rooms: Data<Rooms>, room: Path<String>) -> sse::Sse<sse::ChannelStream> {
    rooms.subscribe(room.as_str()).await
}

#[post("/connect/{room}")]
async fn send_to_room(room: Path<String>, rooms: Data<Rooms>, body: Json<String>) -> HttpResponse {
    rooms.send(room.as_str(), body.as_str()).await;

    HttpResponse::new(StatusCode::OK)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {



    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(connect_to_room)
            .service(send_to_room)
            .app_data(Data::new(Rooms::new()))
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}
