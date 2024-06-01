use std::{fs, sync::{Arc, Mutex, OnceLock}, thread};

use rand::Rng;

use crate::{entity::KCoinfig, events::{event_bus::EventBus, http_message_handler::HttpMessageHandler, log_message_handler::LogMessageHandler, ws_message_handler::WsMessageHandler}, pulgins::forward_task::task_manager::TaskManager, wcferry::{socketio_client::SocketClient, WeChat}};

// 全局参数结构
pub struct GlobalState {
    pub wechat: Option<Arc<Mutex<WeChat>>>,
    pub task_manager: Arc<Mutex<TaskManager>>,
    pub event_bus: Arc<Mutex<EventBus>>,
    pub config: Arc<KCoinfig>,
    pub socketio_client: Option<Arc<tokio::sync::Mutex<SocketClient>>>,
}
// 全局变量
pub static GLOBAL: OnceLock<Arc<GlobalState>> = OnceLock::new();

// 初始化全局变量
pub fn initialize_global() {
    let mut rng = rand::thread_rng();

    // 日志处理器
    let log_handler = Arc::new(LogMessageHandler {  id: rng.gen::<u32>().to_string() });

    // 消息回调处理器
    let http_handler = Arc::new(HttpMessageHandler { id: rng.gen::<u32>().to_string() });

    // ws 消息通讯
    let ws_handler = Arc::new(WsMessageHandler { id: rng.gen::<u32>().to_string() });

    // 消息总线
    let mut event_bus = EventBus::new();
    event_bus.subscribe("ClientMessage", log_handler);
    event_bus.subscribe("ClientMessage", http_handler);
    event_bus.subscribe("ClientMessage", ws_handler);
    let event_bus_arc = Arc::new( Mutex::new(event_bus));   

    // 初始化配置信息
    let k_config: KCoinfig = init_config();

    // 初始化 socketio 客户端
    // 创建ws客户端
    let wsurl = k_config.wsurl.clone();
    let mut socketio_client = None;
    if !wsurl.is_empty() {
        let socket_client = Arc::new(tokio::sync::Mutex::new(SocketClient::new(wsurl.clone())));
        let temp_sc = socket_client.clone();
        tokio::spawn(async move {
            let mut ss = temp_sc.lock().await;
            let _ = ss.connect().await;
        });
        socketio_client = Some(socket_client);
    }

    let global_state: GlobalState = GlobalState{
        wechat: None,
        task_manager: Arc::new(Mutex::new(TaskManager::new(None))),
        event_bus: event_bus_arc,
        config: Arc::new( k_config),
        socketio_client: socketio_client
    };
    let _ = GLOBAL.set(Arc::new(global_state));
}

// 读取配置
fn init_config() -> KCoinfig {

    // 获取应用安装目录的路径
    // let install_dir = resolve_path(&app, ".", None).map_err(|e| e.to_string())?;
    // 定义文件路径
    let file_path = ".\\config.json";
     
    // 尝试创建并写入文件
    let file_str =  fs::read_to_string(&file_path).unwrap();

    let kconfig: KCoinfig = serde_json::from_str(&file_str).unwrap();
    kconfig
}

// 关闭socketio
// if let Some(sc_cliet) = &self.socketio_client {
//     let amst =  sc_cliet.clone();
//     tokio::spawn(async move {
//         let mut ss = amst.lock().await;
//         let _ = ss.disconnect().await;
//     });
// }