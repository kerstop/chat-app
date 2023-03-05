use actix_web_lab::sse;
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use tokio::sync::Mutex;
use tokio::time::Duration;
pub struct Rooms {
    list: Arc<Mutex<HashMap<String, Vec<sse::Sender>>>>,
}

impl Rooms {
    pub fn new() -> Self {
        let rooms = Rooms {
            list: Default::default(),
        };
        tokio::task::spawn(Self::room_cleanup_task(rooms.list.clone()));
        rooms
    }

    async fn room_cleanup_task(handle: Arc<Mutex<HashMap<String, Vec<sse::Sender>>>>) {

        const SLEEP_DURATION: tokio::time::Duration = tokio::time::Duration::from_secs(10);

        while Arc::strong_count(&handle) > 1 {
            {
                let mut list = handle.lock().await;
                let mut rooms_to_delete = Vec::new();

                for (room_name, subscribers) in list.iter() {

                    if subscribers.len() == 0 {
                        rooms_to_delete.push(room_name.clone())
                    }

                }

                for name in rooms_to_delete.iter() {
                    list.remove(name);
                }
            }
            tokio::time::sleep(SLEEP_DURATION).await;
        }
    }

    pub async fn subscribe(&self, room: &str) -> sse::Sse<sse::ChannelStream> {
        const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(10);

        let mut room_list = self.list.lock().await;
        let (tx, rx) = sse::channel(128);

        match room_list.get_mut(room) {
            Some(stream_list) => {
                stream_list.push(tx);
                return rx;
            }
            None => {
                let mut v = Vec::new();
                v.push(tx);
                room_list.insert(room.to_string(), v);
                return rx.with_keep_alive(KEEP_ALIVE_INTERVAL);
            }
        }
    }

    pub async fn send(&self, room: &str, message: &str) {
        let room_list = self.list.lock().await;

        match room_list.get(room) {
            Some(room) => {
                for subscriber in room {
                    subscriber.send(sse::Data::new(message)).await;
                }
            }
            None => return,
        }
    }
}