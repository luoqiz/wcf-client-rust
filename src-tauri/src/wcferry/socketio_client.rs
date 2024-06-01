use std::{sync::{Arc,Mutex}, thread, time::Duration};
use serde_json::{json, Value};
use log::info;
use rust_socketio::{client::Client, ClientBuilder, Payload, RawClient};

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

    pub fn connect(&mut self)   -> std::io::Result<()>{

        let callback = |payload: Payload, socket: RawClient| {
            match payload {
                Payload::String(str) => println!("Received: {}", str),
                Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
                Payload::Text(_) => todo!(),
            }
            socket.emit("test", json!({"got ack": true})).expect("Server unreachable")
        };


        // 发起连接
        let socket = ClientBuilder::new(self.url.clone())
            .namespace("/")
            .on("MSG", callback)
            .on("error", |err, _| eprintln!("Error: {:#?}", err))
            .on("PONG", |payload,_|{ println!("PONG:payload{:#?}", payload)})
            .connect()
            .expect("Connection failed");
     
        let am_client = Arc::new(Mutex::new(socket));
        
        self.client = Some(am_client.clone());
        let task_ping = am_client.clone();
        thread::spawn(  move ||{
            loop {
                let json_payload = json!({"type": "ping"});
                let client = task_ping.lock().unwrap();
                client
                    .emit("PING",json_payload)
                    .expect("Server unreachable");
                 // 睡眠定期推送数据
                thread::sleep(Duration::from_secs(10));
            }
        });
        log::info!("开启websocket {:?}", self.url.clone());
        Ok(())
    }
 
    pub   fn disconnect(&mut self) -> std::io::Result<()> {
        info!("等待 socketIo 断开连接");
        let temp = self.client.clone();
        if let Some(value) = temp {
            let client = value.lock().unwrap();
            let _ = client.disconnect().unwrap();
        }
        info!("socketIo 已断开连接");
        Ok(())
    }

    pub fn send_msg(&mut self, payload: Value){
        let task_msg = self.client.clone();
        if let Some(task_ping) = task_msg {
            thread::spawn(  move ||{
                let client = task_ping.lock().unwrap();
                    client
                        .emit("MSG",payload)
                        .expect("Server unreachable");
            });

        }
    }
}