use async_trait::async_trait;

use crate::global::GLOBAL;

use super::event_handler::{Event, EventHandler};
use serde_json::json;

/// 配置 http 回调地址后，将调用设置的url，
pub struct HttpMessageHandler {
    pub id: String,
}

#[async_trait]
impl EventHandler for HttpMessageHandler {

    async fn handle(&self, event: Event) {
        if let Event::ClientMessage(ref msg) = event {

            let global = GLOBAL.get().unwrap();
            let k_config = global.config.clone();
            let cburl = k_config.cburl.clone();
            if cburl.is_empty() {
                return ;
            }

            log::debug!("http服务 {} 回调地址为: {:?}", self.id, cburl.clone());
            let res = ureq::post(&cburl).send_json(json!(&msg));
            match res {
                Ok(rsp) => {
                    if rsp.status() != 200 {
                        log::error!("转发消息失败，状态码: {}", rsp.status());
                    }
                    log::debug!("{}", rsp.into_string().unwrap());
                }
                Err(e) => {
                    log::error!("转发消息失败：{}", e);
                }
            }
           
        }
    }
}