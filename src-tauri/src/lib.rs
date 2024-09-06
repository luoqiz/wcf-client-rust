// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::sync::{Arc, Mutex};

use chrono::Local;
use log::{info, Level, LevelFilter, Log, Metadata, Record};
use tauri::image::Image;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{command, App, AppHandle, Emitter, Manager, Window, WindowEvent};
use tauri::menu::{MenuBuilder, MenuItemBuilder};
//use tauri::image::Image;
// use tauri::tray::{ TrayIconBuilder, TrayIconEvent,MouseButton, MouseButtonState};
use winapi::{
    //shared::winerror::ERROR_ALREADY_EXISTS,
    um::{
        // errhandlingapi::GetLastError,
        synchapi::CreateMutexA,
        winuser::{FindWindowA, SetForegroundWindow, ShowWindow, SW_RESTORE},
    },
};

use global::initialize_global;
use http_server::HttpServer;

mod endpoints;
mod entity;
mod events;
mod global;
mod http_server;
mod pulgins;
mod tauri_commands;
mod wcferry;

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
    http_server: HttpServer,
    // config: KCoinfig,
}

// 开启http_server
#[command]
async fn start_server(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    host: String,
    port: u16,
    cburl: String,
    wsurl: String,
) -> Result<(), String> {
    let host_bytes = host
        .split('.')
        .map(|part| part.parse::<u8>().unwrap_or(0))
        .collect::<Vec<u8>>()
        .try_into()
        .map_err(|_| "Invalid host address".to_string())?;
    {
        let mut app_state = state.inner().lock().unwrap();
        let _ = app_state.http_server.start(host_bytes, port, cburl, wsurl);
    }

    info!("服务启动，监听 http://{}:{}", host, port);
    info!("浏览器访问 http://localhost:{}/swagger/ 查看文档", port);
    Ok(())
}

// 关闭 http_server
#[command]
async fn stop_server(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<(), String> {
    {
        let mut app_state = state.inner().lock().unwrap();
        app_state.http_server.stop()?;
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
            http_server: HttpServer::new(),
            // config: KCoinfig::read(),
        })))
        .invoke_handler(tauri::generate_handler![
          start_server,
          stop_server,
          confirm_exit,
          tauri_commands::get_contacts,
          tauri_commands::get_user_info,
          tauri_commands::write_wxid_task,
          tauri_commands::read_wxid_task,
          tauri_commands::save_config,
          tauri_commands::read_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
