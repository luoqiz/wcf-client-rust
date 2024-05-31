use std::{collections::HashMap, sync::Arc};

use super::event_handler::{Event, EventHandler};

pub struct EventBus {
    handlers: HashMap<String, Vec<Arc<dyn EventHandler + Send + Sync>>>,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus {
            handlers: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, event_type: &str, handler: Arc<dyn EventHandler + Send + Sync>) {
        self.handlers
            .entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(handler);
    }



    pub async fn publish(&self, event: Event) {
        let event_type = match &event {
            Event::ClientMessage(_) => "ClientMessage",
        };

         // 克隆处理器列表以避免生命周期问题
         if let Some(handlers) = self.handlers.get(event_type).cloned() {
            for handler in handlers {
                let event = event.clone();
                tokio::spawn(async move {
                    handler.handle(event).await;
                });
            }
        }
    }
}
