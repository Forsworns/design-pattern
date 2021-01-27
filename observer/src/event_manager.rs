use crate::{EventListener, HashMap, LotteryResult};

#[derive(Hash, PartialEq, Eq)]
pub enum EventType {
    Mq,
    Message,
}

pub struct EventManager {
    pub listeners: HashMap<EventType, Vec<Box<dyn EventListener>>>,
}

impl EventManager {
    pub fn new() -> Self {
        let mut listeners = HashMap::new();
        listeners.insert(EventType::Message, vec![]);
        listeners.insert(EventType::Mq, vec![]);
        Self { listeners }
    }

    pub fn subscribe(&mut self, event: EventType, listener: impl EventListener + 'static) {
        let users = self.listeners.get_mut(&event).unwrap();
        users.push(Box::new(listener));
    }

    pub fn unsubscribe(&mut self, event: EventType, listener: impl EventListener + 'static) {
        let users = self.listeners.get_mut(&event).unwrap();
        let pos = users.iter().position(|x| *(*x) == listener).unwrap();
        users.remove(pos);
    }

    pub fn notify(&mut self, event: EventType, result: LotteryResult) {
        let users = self.listeners.get(&event).unwrap();
        for listener in users {
            listener.do_event(result.clone());
        }
    }
}

#[macro_export]
macro_rules! envent_manager {
    () => {
        $crate::event_manager::EventManager::new()
    };
    ($($operation:expr)*, $(,)?) => {
        let event_manager = EventManager::new();
        {event_manager.listeners.insert(operation, Vec::new());}+
        event_manager
    };
}
