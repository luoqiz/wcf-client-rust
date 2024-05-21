use std::{sync::{Arc, Mutex}};
use futures_util::future::ok;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::command;
use crate::{task_file::{self, Task}, wcferry::{wcf::RpcContacts, WeChat}, wechat_api_handler, AppState};


#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq,)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    status: u16,
    error: Option<String>,
    data: Option<T>,
}

#[macro_export]
macro_rules! wechat_api_handler_inner {
    ($wechat:expr, $handler:expr, $desc:expr) => {{
        let wechat = $wechat.lock().unwrap();
        let result: Result<_, _> = $handler(&*wechat);
        match result {
            Ok(data) => json!(&ApiResponse {
                status: 0,
                error: None,
                data: Some(data),
            }),
            Err(error) => json!(&ApiResponse::<()> {
                status: 1,
                error: Some(format!("{}失败: {}", $desc, error)),
                data: None,
            }),
        }
    }};
    ($wechat:expr, $handler:expr, $param:expr, $desc:expr) => {{
        let wechat = $wechat.lock().unwrap();
        let result: Result<_, _> = $handler(&*wechat, $param);
        match result {
            Ok(data) => Ok(json!(&ApiResponse {
                status: 0,
                error: None,
                data: Some(data),
            })),
            Err(error) => Ok(json!(&ApiResponse::<()> {
                status: 1,
                error: Some(format!("{}失败: {}", $desc, error)),
                data: None,
            })),
        }
    }};
}

#[command]
pub async fn get_contacts(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<Value, String> {
    let app_state = state.inner().lock().unwrap();
    let wechat = &app_state.http_server.wechat.clone().unwrap();
    let res: Value = wechat_api_handler_inner!(wechat, WeChat::get_contacts, "获取所有联系人");
    Ok(res)
}

#[command]
pub async fn get_user_info(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<Value, String> {
    let app_state = state.inner().lock().unwrap();
    let wechat = &app_state.http_server.wechat.clone().unwrap();
    let res: Value = wechat_api_handler_inner!(wechat, WeChat::get_user_info, "获取登录账号信息");
    Ok(res)
}

#[command]
pub fn write_wxid_task(wxid: &str,task: Task){
    log::info!("{:?}",wxid);
    let _ = task_file::write_to_json_file(wxid, &task);
}

#[command]
pub fn read_wxid_task(wxid: &str) -> Result<Vec<Task>, String>{
    log::info!("{:?}",wxid);
    let res = task_file::read_from_json_file(wxid);
    Ok(res)
}