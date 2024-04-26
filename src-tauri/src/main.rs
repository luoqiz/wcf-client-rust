// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


use chrono::Local;
use log::{info, Level, LevelFilter, Log, Metadata, Record};
use std::ptr;
use std::sync::{Arc, Mutex};
use tauri::{command, AppHandle, Manager};
use winapi::{
    shared::winerror::ERROR_ALREADY_EXISTS,
    um::{
        // errhandlingapi::GetLastError,
        synchapi::CreateMutexA,
        winuser::{FindWindowA, SetForegroundWindow, ShowWindow, SW_RESTORE},
    },
};

mod endpoints;
mod http_server;
mod wcferry;
use http_server::HttpServer;

struct FrontendLogger {
    app_handle: tauri::AppHandle,
}

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
}

#[command]
async fn start_server(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    host: String,
    port: u16,
    cburl: String,
) -> Result<(), String> {
    let host_bytes = host
        .split('.')
        .map(|part| part.parse::<u8>().unwrap_or(0))
        .collect::<Vec<u8>>()
        .try_into()
        .map_err(|_| "Invalid host address".to_string())?;

    {
        let mut app_state = state.inner().lock().unwrap();
        app_state.http_server.start(host_bytes, port, cburl)?;
    }

    info!("服务启动，监听 http://{}:{}", host, port);
    info!("浏览器访问 http://localhost:{}/swagger/ 查看文档", port);
    Ok(())
}

#[command]
async fn stop_server(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<(), String> {
    {
        let mut app_state = state.inner().lock().unwrap();
        app_state.http_server.stop()?;
    }

    info!("服务停止");
    Ok(())
}

#[command]
async fn confirm_exit(app_handle: tauri::AppHandle) {
    let _ = stop_server(app_handle.state()).await;
    std::process::exit(0);
}

// fn handle_system_tray_event(app_handle: &tauri::AppHandle, event: tauri::SystemTrayEvent) {
//     match event {
//         tauri::SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
//             "quit" => {
//                 app_handle.emit_all("request-exit", ()).unwrap();
//             }
//             _ => {}
//         },
//         tauri::SystemTrayEvent::LeftClick { .. } => {
//             if let Some(window) = app_handle.get_window("main") {
//                 window.show().unwrap();
//                 window.set_focus().unwrap();
//             }
//         }
//         _ => {}
//     }
// }

fn init_window(window: tauri::Window) {
    window.hide().unwrap();
    if let Ok(Some(monitor)) = window.primary_monitor() {
        let monitor_size = monitor.size();
        if let Ok(window_size) = window.outer_size() {
            let x = (monitor_size.width as i32 - window_size.width as i32) / 2;
            let y = (monitor_size.height as i32 - window_size.height as i32) / 2;
            window
                .set_position(tauri::Position::Logical(tauri::LogicalPosition {
                    x: x.into(),
                    y: y.into(),
                }))
                .unwrap();
        } else {
            let x = (monitor_size.width as i32 - 640) / 2;
            let y = (monitor_size.height as i32 - 320) / 2;
            window
                .set_position(tauri::Position::Logical(tauri::LogicalPosition {
                    x: x.into(),
                    y: y.into(),
                }))
                .unwrap();
        }
    }
    window.show().unwrap();
}

fn init_log(handle: AppHandle) {
    log::set_boxed_logger(Box::new(FrontendLogger { app_handle: handle }))
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("Failed to initialize logger");
}

fn main() {
    
    // let mutex_name = b"Global\\wcfrust_app_mutex\0";
    // unsafe {
    //     let handle = CreateMutexA(ptr::null_mut(), 0, mutex_name.as_ptr() as *const i8);
    //     if handle.is_null() {
    //         eprintln!("Failed to create mutex.");
    //         return;
    //     }
    //     if GetLastError() == ERROR_ALREADY_EXISTS {
    //         let window_name = "WcfRust\0".as_ptr() as *const i8;
    //         let hwnd = FindWindowA(ptr::null(), window_name);
    //         if !hwnd.is_null() {
    //             ShowWindow(hwnd, SW_RESTORE);
    //             SetForegroundWindow(hwnd);
    //         }
    //         return;
    //     }
    // }

    // let quit = CustomMenuItem::new("quit".to_string(), "退出");
    // let tray_menu = SystemTrayMenu::new().add_item(quit);
    // let tray = SystemTray::new().with_menu(tray_menu);

     let app = tauri::Builder::default()
        // .setup(|app| {
        //     init_window(app.get_window("main").unwrap());
        //     init_log(app.app_handle());
        //     Ok(())
        // })
        .plugin(tauri_plugin_shell::init())
        // .on_window_event(move |event| match event.event() {
        //     WindowEvent::CloseRequested { api, .. } => {
        //         api.prevent_close();
        //         if let Some(window) = event.window().get_window("main") {
        //             window.hide().unwrap();
        //         }
        //     }
        //     _ => {}
        // })
        // .system_tray(tray)
        // .on_system_tray_event(handle_system_tray_event)
        .manage(Arc::new(Mutex::new(AppState {
            http_server: HttpServer::new(),
        })))
        .invoke_handler(tauri::generate_handler![start_server, stop_server, confirm_exit]);

    app.run(tauri::generate_context!())
        .expect("error while running tauri application");
}
