use std::sync::{Arc, Mutex};
use async_trait::async_trait;

use crate::wcferry::WeChat;

#[derive(Clone)]
pub enum Event {
    ClientMessage(String),
}

#[async_trait]
pub trait EventHandler {
    async fn handle(&self, event: Event);
}

pub struct ClientMessageHandler {
    pub id: String,
}

#[async_trait]
impl EventHandler for ClientMessageHandler {
    async fn handle(&self, event: Event) {
        if let Event::ClientMessage(ref msg) = event {
            log::error!("Handler {} received message: {}", self.id, msg);
            let response = format!("Handler {} processed: {}", self.id, msg);

            // let mut stream = stream.lock().await;
            // if let Err(e) = stream.write_all(response.as_bytes()).await {
            //     eprintln!("Handler {} failed to write to socket; err = {:?}", self.id, e);
            // }
        }
    }
}
