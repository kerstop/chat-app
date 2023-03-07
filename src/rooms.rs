use actix::{Actor, StreamHandler, Addr, Message};
use actix_web::{HttpResponse, Error, web, HttpRequest};
use actix_web_actors::ws;
use tokio_stream::Stream;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use std::collections::HashMap;
use tokio::sync::{Mutex, RwLock};
use tokio::sync::broadcast;
use tokio::time::Duration;

struct Subscriber{
    tx: broadcast::Sender<ChatMessage>,
}

impl Actor for Subscriber {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        <Self as StreamHandler<BroadcastResult>>::add_stream(BroadcastStream::new(self.tx.subscribe()), ctx);
    } 
}

impl StreamHandler<WebsocketResult> for Subscriber {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(txt)) => println!("recieved msg: {txt}"),
            Ok(ws::Message::Close(_)) => (),
            _ => (),
        }
    }
}

impl StreamHandler<BroadcastResult> for Subscriber {
    fn handle(&mut self, msg: Result<ChatMessage, BroadcastStreamRecvError>, ctx: &mut Self::Context) {
        match msg {
            Ok(msg) => ctx.text(msg.txt),
            Err(x) => println!("A client missed {} messages", x.to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct ChatMessage {
    txt: String
}

impl Message for ChatMessage {
    type Result = &'static str;
}

type BroadcastResult = Result<ChatMessage, BroadcastStreamRecvError>;
type WebsocketResult = Result<ws::Message, ws::ProtocolError>;

pub struct Rooms {
    list: RwLock<HashMap<String, broadcast::Sender<ChatMessage>>>,
}

impl Rooms {
    pub fn new() -> Self {
        let rooms = Rooms {
            list: Default::default(),
        };
        rooms
    }

    pub async fn subscribe(&self, room: &str, req: &HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
        const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(10);

        match self.list.read().await.get(room) {
            Some(room_stream) => {
                return ws::start(Subscriber { tx: room_stream.clone() }, req, stream);
            }
            None => {
                let (tx, rx) = broadcast::channel(1024);
                self.list.write().await.insert(room.to_string(), tx.clone());
                return ws::start(Subscriber { tx }, req, stream);

            }
        }
    }

    pub async fn send(&self, room: &str, message: &str) {

        match self.list.read().await.get(room) {
            Some(room) => {
                room.send(ChatMessage { txt: message.to_string() });
            }
            None => return,
        }
    }
}