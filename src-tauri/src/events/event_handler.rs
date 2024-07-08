use async_trait::async_trait;

use crate::wcferry::wcf;

#[derive(Clone)]
pub enum Event {
    ClientMessage(wcf::WxMsg),
    SocketIoMessage(bool),
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
            log::info!("Handler {} received message: {:?}", self.id, msg);
        }
    }
}
