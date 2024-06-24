use std::fs::{self, File};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::command;
use crate::entity::KCoinfig;
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
    let app_state = state.inner().lock().unwrap();
    let task_manager_lock = &app_state.http_server.task_manager.clone();
    let mut task_manager = task_manager_lock.lock().unwrap();
    let _ = task_manager.add_or_remove_task(wxid, Some(task),None);
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

#[command]
pub fn save_config(state: tauri::State<'_, Arc<Mutex<AppState>>>, config: KCoinfig) -> Result<bool, String>{
    
    // 获取应用安装目录的路径
    // let install_dir = resolve_path(&app, ".", None).map_err(|e| e.to_string())?;
    // 定义文件路径
    let file_path = ".\\config.json";
    // let file_path = "e:\\wcf-client-rust\\config.json5";
    // log::info!("----file_path-----{:?}",file_path);
    
    // 生成完整的文件路径
    // let file_path = file_path.join("config.json5");

    // 尝试创建并写入文件
    let mut file = File::create(&file_path).map_err(|e| e.to_string())?;
    let json_str = serde_json::to_string(&config).unwrap();
    file.write_all(json_str.as_bytes()).map_err(|e| e.to_string())?;

    Ok(true)
}

#[command]
pub fn read_config(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<KCoinfig, String>{
    
    // 获取应用安装目录的路径
    // let install_dir = resolve_path(&app, ".", None).map_err(|e| e.to_string())?;
    // 定义文件路径
    let file_path = ".\\config.json";
     
    // 尝试创建并写入文件
    let file_str =  fs::read_to_string(&file_path).unwrap();

    let kconfig: KCoinfig = serde_json::from_str(&file_str).unwrap();
    Ok(kconfig)
}