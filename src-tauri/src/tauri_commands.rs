use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::command;
use crate::{pulgins::forward_task::task_file::Task, wcferry::WeChat, AppState};


#[derive(Serialize, Deserialize, Clone, PartialEq,)]
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
    ($data:expr) => {{
        Ok(json!(&ApiResponse {
            status: 0,
            error: None,
            data: Some(data),
        }))
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
pub fn write_wxid_task(state: tauri::State<'_, Arc<Mutex<AppState>>>, wxid: &str, task: Task)->Result<String,String>{
    log::info!("{:?}",wxid);
    let app_state = state.inner().lock().unwrap();
    let task_manager_lock = &app_state.http_server.task_manager.clone();
    let mut task_manager = task_manager_lock.lock().unwrap();
    let _ = task_manager.add_or_remove_task(Some(task),None);
    Ok("ok".to_string())
}

#[command]
pub fn read_wxid_task(state: tauri::State<'_, Arc<Mutex<AppState>>>, wxid: &str) -> Result<Vec<Task>, String>{
    log::info!("{:?}",wxid);
    let app_state = state.inner().lock().unwrap();
    let task_manager_lock = &app_state.http_server.task_manager.clone();
    let task_manager = task_manager_lock.lock().unwrap();
    let tasks = &task_manager.tasks;
    //wechat_api_handler_inner!(task_manager.tasks)
    Ok(tasks.clone())
}