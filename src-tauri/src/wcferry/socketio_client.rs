use std::{sync::{Arc}, thread, time::Duration};
use serde_json::json;
use log::{debug};
use futures_util::{lock::Mutex, FutureExt};
use rust_socketio::{
    asynchronous::{Client, ClientBuilder},
    Payload,
};

#[derive(Clone)]
pub struct SocketClient {
    pub client: Arc<Mutex<Client>>,
}

impl SocketClient {
    pub async fn new(cburl: String) -> Self {
        let callback = |payload: Payload, socket: Client| {
            async move {
                match payload {
                    Payload::Text(values) => log::info!("Received: {:#?}", values),
                    Payload::Binary(bin_data) => log::info!("Received bytes: {:#?}", bin_data),
                    _ => {}
                }
            }
                .boxed()
        };
        // 发起连接
        let socket = ClientBuilder::new(cburl)
            // .namespace("/")
            .on("MSG", callback)
            .on("error", |err, _| {
                async move { eprintln!("Error: {:#?}", err) }.boxed()
            })
            .on("PONG", |_,_|{
                async move { debug!("服务器返回pong信息")}.boxed()
            })
            .connect()
            .await
            .expect("Connection failed");
     
        let am_client = Arc::new(Mutex::new(socket));
        let socket_client = SocketClient{
            client: am_client.clone()
        };

        let task_ping = am_client.clone();
        tokio::spawn(async move {
            loop {
                let json_payload = json!({"type": "ping"});
                let client = task_ping.lock().await;
                client
                    .emit("PING",json_payload)
                    .await
                    .expect("Server unreachable");
                 // 睡眠定期推送数据
                 thread::sleep(Duration::from_secs(10));
            }
        });
        socket_client
    }
 
    pub fn disconnect(&mut self) -> Result<(), String> {
        let temp = self.client.clone();
        tokio::spawn(async move{
            let _ = temp.try_lock().unwrap().disconnect().await;
        });
        debug!("socketIo client stopped");
        Ok(())
    }
}