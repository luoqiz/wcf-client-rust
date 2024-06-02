use async_trait::async_trait;
use serde_json::json;

use super::event_handler::{Event, EventHandler};
use crate::global::GLOBAL;

/// 日志打印
pub struct WsMessageHandler {
    pub id: String,
}

#[async_trait]
impl EventHandler for WsMessageHandler {

    async fn handle(&self, event: Event) {
        if let Event::ClientMessage(ref msg) = event {
            log::debug!("ws处理器 {} -- 接收到信息", self.id);
            let global = GLOBAL.get().unwrap();
            let socket_arc = global.socketio_client.clone();
            if let Some(client) = socket_arc {
                let mut socket = client.lock().unwrap();
                socket.send_msg(json!(msg));
            }
        }
    }
}