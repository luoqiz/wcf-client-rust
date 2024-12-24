#![recursion_limit = "256"]

use std::sync::{Arc, Mutex};
use std::fs::{self, File};
use std::io::Write;
use std::ptr;

use chrono::Local;
use handler::event_entity::Event;
use local_ip_address::local_ip;
use log::{info, Level, LevelFilter, Log, Metadata, Record};
use tauri::image::Image;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{command, App, AppHandle, Emitter, Manager, Window, WindowEvent};
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use service::global_service::{initialize_global, GLOBAL};
use wechat_config::WechatConfig;


use winapi::{
    shared::winerror::ERROR_ALREADY_EXISTS,
    um::{
        errhandlingapi::GetLastError,
        synchapi::CreateMutexA,
        winuser::{FindWindowA, SetForegroundWindow, ShowWindow, SW_RESTORE},
    },
};
// use tauri::{ CustomMenuItem,   SystemTray, SystemTrayMenu};

mod endpoints;
mod wcferry;
mod service;
mod wechat_config;
mod handler;
// mod tauri_commands;

struct FrontendLogger {
    app_handle: tauri::AppHandle,
}

// 日志打印
impl Log for FrontendLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!(
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            );
            self.app_handle.emit("log-message", msg).unwrap();
        }
    }

    fn flush(&self) {}
}

struct AppState {
    http_server_running: bool,
}


#[tauri::command]
async fn ip() -> Result<String, String> {
    let global = GLOBAL.get().unwrap();
    let wechat_config = global.wechat_config.read().unwrap();
    let local = local_ip().map_err(|e| e.to_string())?;
    Ok(String::from(local.to_string()+ ":" + &wechat_config.http_server_port.to_string()))
}

#[tauri::command]
async fn is_http_server_running(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<bool, String> {
    let app_state = state.inner().lock().map_err(|e| e.to_string())?;
    Ok(app_state.http_server_running)
}



// 写入配置到文件中
#[command]
fn save_wechat_config(
    config: WechatConfig,
) -> Result<bool, String> {
    // 定义文件路径
    let file_path = ".\\config.json5";

    // 尝试创建并写入文件
    let mut file = File::create(&file_path).map_err(|e| e.to_string())?;
    let json_str = serde_json::to_string(&config).unwrap();
    file.write_all(json_str.as_bytes())
        .map_err(|e| e.to_string())?;
    let global = GLOBAL.get().unwrap();
    let mut wechat_config_lock = global.wechat_config.write().unwrap();
    wechat_config_lock.cburl = config.cburl.clone();
    wechat_config_lock.wsurl = config.wsurl.clone();
    wechat_config_lock.file_dir = config.file_dir.clone();
    wechat_config_lock.http_server_port = config.http_server_port.clone();
    wechat_config_lock.front_msg_show = config.front_msg_show.clone();
    info!("Wechat configuration update {:?}", serde_json::to_string(&config));
    Ok(true)
}


// 读取文件
#[command]
fn read_wechat_config() -> Result<WechatConfig, String> {
    // 获取应用安装目录的路径
    // let install_dir = resolve_path(&app, ".", None).map_err(|e| e.to_string())?;
    // 定义文件路径
    let file_path = ".\\config.json5";

    // 尝试创建并写入文件
    let file_str = fs::read_to_string(&file_path).unwrap();

    let wechatconfig: WechatConfig = serde_json::from_str(&file_str).unwrap();
    Ok(wechatconfig)
}


#[command]
async fn start_server(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    _host: String,
    _port: u16,
) -> Result<(), String> {
    {
        let mut app_state = state.inner().lock().unwrap();
        if !app_state.http_server_running {
          // 发送到消息监听器中
          let global = GLOBAL.get().unwrap();
          let event_bus = global.startup_event_bus.lock().unwrap();
          let _ = event_bus.send_message(Event::StartUp());
          app_state.http_server_running = true;
        }
    }
    Ok(())
}

#[command]
async fn stop_server(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<(), String> {
    {
        let mut app_state = state.inner().lock().map_err(|e| e.to_string())?;
        if app_state.http_server_running {
            let global = GLOBAL.get().unwrap();
            let event_bus = global.startup_event_bus.lock().unwrap();
            let _ = event_bus.send_message(Event::Shutdown());
            app_state.http_server_running = false;
        } else {
            info!("服务已停止");
        }

        
    }

    info!("服务停止");
    Ok(())
}

// 确认退出
#[command]
async fn confirm_exit(app_handle: tauri::AppHandle) {
    let _ = stop_server(app_handle.state()).await;
    std::process::exit(0);
}

// 处理系统托盘菜单
fn handle_system_tray_event(window: &Window, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            if let Some(window) = window.get_webview_window("main") {
                window.hide().unwrap();
            }
        }
        _ => {}
    }
}

// 初始化日志功能
fn init_log(handle: AppHandle) {
    log::set_boxed_logger(Box::new(FrontendLogger { app_handle: handle }))
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("Failed to initialize logger");
}

// 初始化菜单
fn init_menu(app: &mut App) {
    let toggle = MenuItemBuilder::with_id("quit", "退出").build(app).unwrap();
    let menu = MenuBuilder::new(app).items(&[&toggle]).build().unwrap();
    let _ = TrayIconBuilder::new()
        .menu(&menu)
        .icon(Image::from_path("icons/icon.png").unwrap())
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "quit" => {
                app.exit(0);
            }
            _ => (),
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                id: _,
                position: _,
                rect: _,
            } = event
            {
                let app: &AppHandle = tray.app_handle();
                if let Some(webview_window) = app.get_webview_window("main") {
                    if webview_window.is_visible().unwrap() {
                        let _ = webview_window.hide();
                    } else {
                        let _ = webview_window.show();
                        let _ = webview_window.set_focus();
                    }
                }
            }
        })
        .build(app);
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
          // init_window(app.get_webview_window("main").unwrap());
          init_log(app.app_handle().clone());
          init_menu(app);
          initialize_global();
          Ok(())
        })
        .on_window_event(handle_system_tray_event)
        .manage(Arc::new(Mutex::new(AppState {
            http_server_running: false,
        })))
        .invoke_handler(tauri::generate_handler![
            start_server,
            stop_server,
            confirm_exit,
            is_http_server_running,
            ip,
            save_wechat_config,
            read_wechat_config,
        //   tauri_commands::get_contacts,
        //   tauri_commands::get_user_info,
        //   tauri_commands::write_wxid_task,
        //   tauri_commands::read_wxid_task,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
