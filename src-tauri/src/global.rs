use std::sync::{Arc, Mutex, OnceLock};

use rand::Rng;

use crate::{events::{event_bus::EventBus, event_handler::ClientMessageHandler}, pulgins::forward_task::task_manager::TaskManager, wcferry::WeChat};

// 全局参数结构
pub struct GlobalState {
    pub wechat: Option<Arc<Mutex<WeChat>>>,
    pub task_manager: Arc<Mutex<TaskManager>>,
    pub event_bus: Arc<Mutex<EventBus>>,
}
// 全局变量
pub static GLOBAL: OnceLock<Arc<GlobalState>> = OnceLock::new();

// 初始化全局变量
pub fn initialize_global() {
     // 日志处理器
     let mut rng = rand::thread_rng();
     let id = rng.gen::<u32>().to_string();
     let log_handler = Arc::new(ClientMessageHandler { id });

    let mut event_bus = EventBus::new();
    event_bus.subscribe("ClientMessage", log_handler);
    let event_bus_arc = Arc::new( Mutex::new(event_bus));   

    let global_state: GlobalState = GlobalState{
        wechat: None,
        task_manager: Arc::new(Mutex::new(TaskManager::new(None))),
        event_bus: event_bus_arc,
    };
    let _ = GLOBAL.set(Arc::new(global_state));
}