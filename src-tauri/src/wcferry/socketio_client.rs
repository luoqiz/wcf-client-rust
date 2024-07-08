use log::info;
use rust_socketio::{client::Client, ClientBuilder, Payload, RawClient};
use serde_json::{json, Value};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{global::GLOBAL, wcferry::wcf};

#[derive(Clone)]
pub struct SocketClient {
    pub url: String,
    pub client: Option<Arc<Mutex<Client>>>,
}

impl SocketClient {
    pub fn new(wsurl: String) -> Self {
        let socket_client = SocketClient {
            url: wsurl,
            client: Option::None,
        };
        socket_client
    }

    pub fn connect(&mut self) -> std::io::Result<()> {
        let callback = |payload: Payload, socket: RawClient| {
            match payload {
                Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
                Payload::Text(_) => todo!(),
                _ => (),
            }
            socket
                .emit("test", json!({"got ack": true}))
                .expect("Server unreachable")
        };

        let func_send_rich_txt = |payload: Payload, _| match payload {
            Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
            Payload::Text(res) => {
                log::info!("---- {:?}", res);
                let rich_msg_vec: Result<Vec<wcf::RichText>, serde_json::Error> = res
                    .into_iter()
                    .map(|value| serde_json::from_value(value))
                    .collect();
                let global = GLOBAL.get().unwrap();

                for rich_text in rich_msg_vec.unwrap() {
                    let wechat_arc = global.wechat.clone();
                    let wechat1 = wechat_arc.lock().unwrap();
                    {
                        let wechat2 = wechat1.clone().unwrap();
                        let _ = wechat2.lock().unwrap().send_rich_text(rich_text);
                    }
                }
            }
            _ => (),
        };

        // 发起连接
        let socket = ClientBuilder::new(self.url.clone())
            .namespace("/")
            .on("MSG", callback)
            .on("error", |err, _| eprintln!("Error: {:#?}", err))
            .on("PONG", |payload, _| println!("PONG:payload{:#?}", payload))
            .on("FuncSendRichTxt", func_send_rich_txt)
            .connect()
            .expect("Connection failed");

        let am_client = Arc::new(Mutex::new(socket));

        self.client = Some(am_client.clone());
        let task_ping = am_client.clone();
        thread::spawn(move || {
            loop {
                let json_payload = json!({"type": "ping"});
                let client = task_ping.lock().unwrap();
                client
                    .emit("PING", json_payload)
                    .expect("Server unreachable");
                // 睡眠定期推送数据
                thread::sleep(Duration::from_secs(10));
            }
        });
        log::info!("开启websocket {:?}", self.url.clone());
        Ok(())
    }

    pub fn disconnect(&mut self) -> std::io::Result<()> {
        info!("等待 socketIo 断开连接");
        let temp = self.client.clone();
        if let Some(value) = temp {
            let client = value.lock().unwrap();
            let _ = client.disconnect().unwrap();
        }
        info!("socketIo 已断开连接");
        Ok(())
    }

    pub fn send_msg(&mut self, payload: Value) {
        let task_msg = self.client.clone();
        if let Some(task_ping) = task_msg {
            thread::spawn(move || {
                let client = task_ping.lock().unwrap();
                client.emit("MSG", payload).expect("Server unreachable");
            });
        }
    }
}
