use std::thread;
use serde_json::json;
use std::time::Duration;
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use log::{debug, error};
use rust_socketio::{
    asynchronous::{Client, ClientBuilder},
    Payload,
};
use tokio::io;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use crate::wcferry::WeChat;

pub struct SocketClient {
    pub client: Option<Client>,
    pub wechat: Option<Arc<Mutex<WeChat>>>,
}

impl SocketClient {
    pub fn new() -> Self {
        SocketClient {
            client: None,
            wechat: None,
        }
    }


    pub async fn start(&mut self, host: [u8; 4], port: u16, cburl: String) -> Result<(), String> {

        // 初始化微信客户端连接
        let wechat = Arc::new(Mutex::new(WeChat::new(true, cburl)));
        self.wechat = Some(wechat.clone());

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
        let mut socket = ClientBuilder::new("http://127.0.0.1:9001/?userId=".to_owned() + &*user_id.to_string())
            // .namespace("/")
            .on("SINGLE_CHAT", callback)
            .on("error", |err, _| {
                async move { eprintln!("Error: {:#?}", err) }.boxed()
            })
            .connect()
            .await
            .expect("Connection failed");
        self.client = Some(socket);
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
        if let Some(wechat) = &self.wechat {
            let mut wechat = wechat.lock().unwrap(); // 获取 Mutex 的锁
            wechat.stop().unwrap();
        }
        debug!("socketIo client stopped");
        Ok(())
    }
}