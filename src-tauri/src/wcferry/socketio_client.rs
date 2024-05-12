use std::thread;
use serde_json::json;
use std::time::Duration;
use futures_util::FutureExt;
use log::{debug, error};
use rust_socketio::{
    asynchronous::{Client, ClientBuilder},
    Payload,
};
use tokio::io;
use tokio::task::JoinHandle;

#[derive(Clone)]
pub struct SocketClient {
    pub client: Option<Client>,
}

impl SocketClient {
    pub fn new() -> Self {
        SocketClient {
            client: None,
        }
    }

    // 启动 socket 服务
    pub async fn start(&mut self, cburl: String) -> Result<(), String> {

        let callback = |payload: Payload, socket: Client| {
            async move {
                match payload {
                    Payload::Text(values) => println!("Received: {:#?}", values),
                    Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
                    _ => {}
                }
            }
                .boxed()
        };
        // 发起连接
        let mut socket = ClientBuilder::new(cburl.clone())
            // .namespace("/")
            .on("SINGLE_CHAT", callback)
            .on("error", |err, _| {
                async move { eprintln!("Error: {:#?}", err) }.boxed()
            })
            .connect()
            .await
            .expect("Connection failed");
        self.client = Some(socket.clone());
        let user_id="789";
        let to_user_id = "123";
        let handle: JoinHandle<io::Result<()>> = tokio::spawn(async move {
            loop {
                // 睡眠定期推送数据
                thread::sleep(Duration::new(20, 0));
                let json_payload = json!({"type":23,"messageId":34,"content":"ping","userId":user_id,"fromUserId":user_id,"toUserId":to_user_id});
                socket
                    .emit("SINGLE_CHAT", json_payload)
                    .await
                    .expect("Server unreachable");
            }
        });

        handle.await.expect("error");
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), String> {
        if let Some(client)  = self.client.take() {
            tokio::spawn(async move {
                client.disconnect().await.expect("Disconnect failed");
            });
        }
        debug!("socketIo client stopped");
        Ok(())
    }
}