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
            log::info!("ws处理器 {} -- 接收到信息: {:?}", self.id, msg);

            let global = GLOBAL.get().unwrap();
            let socket_arc = &global.socketio_client;
            if let Some(client) = socket_arc {
                let mut socket = client.lock().await;
                socket.send_msg(json!(msg)).await;
            }
            

            // let rt = Runtime::new().unwrap();
            //     rt.block_on(async move{
            //         let mut socket = client.lock().await;
            //         socket.send_msg(json!(msg)).await;
            //     });

             // 异步发不了消息，暂不了解原因
                // let scc = client.clone();
                // rt.spawn( async move{
                //     let mut socket = scc.lock().await;
                //     socket.send_msg(json!(msg)).await;
                // });
        }
    }
}