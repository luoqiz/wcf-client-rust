use std::{sync::{Arc}, thread, time::Duration};
use serde_json::{json, Value};
use log::{debug};
use futures_util::{lock::Mutex, FutureExt};
use rust_socketio::{
    asynchronous::{Client, ClientBuilder},
    Payload,
};

#[derive(Clone)]
pub struct SocketClient {
    pub url: String,
    pub client: Option<Arc<Mutex<Client>>>,
}

impl SocketClient {
    pub fn new(wsurl: String) -> Self {
        let socket_client = SocketClient{
            url: wsurl ,
            client: Option::None
        };
        socket_client
    }

    pub async fn connect(&mut self)   -> std::io::Result<()>{
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
        let socket = ClientBuilder::new(self.url.clone())
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
        
        self.client = Some(am_client.clone());
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
        log::info!("开启websocket {:?}", self.url.clone());
        Ok(())
    }
 
    pub fn disconnect(&mut self) -> Result<(), String> {
        let temp = self.client.clone();
        if let Some(value) = temp {
            tokio::spawn(async move{
                let _ = value.try_lock().unwrap().disconnect().await;
            });
        }
        debug!("socketIo client stopped");
        Ok(())
    }

    pub async fn send_msg(&mut self, payload: Value){
        log::info!("ws发送消息: {:?}--",payload);
        let task_msg = self.client.clone();
        if let Some(value) = task_msg {
            // tokio::spawn(async move {
                // let json_payload1 = json!({"type": "msg"});
                let client = value.lock().await;
                client.emit("MSG",payload)
                    .await
                    .expect("Server unreachable");
            // });
        }
    }
}