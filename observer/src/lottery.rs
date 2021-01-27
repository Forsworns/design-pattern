use crate::{EventManager, EventType, MessageEventListener, MqEventListener};

#[derive(Debug, Clone)]
pub struct LotteryResult {
    u_id: String,
    result: String,
}

impl LotteryResult {
    pub fn new(u_id: String, result: String) -> Self {
        Self { u_id, result }
    }
}

pub struct LotteryService {
    manager: EventManager,
}

impl LotteryService {
    pub fn new() -> Self {
        let mut manager = EventManager::new();
        manager.subscribe(EventType::Message, MessageEventListener {});
        manager.subscribe(EventType::Mq, MqEventListener {});
        Self { manager }
    }

    pub fn draw(&mut self, u_id: String) -> LotteryResult {
        let result = generate_result(u_id);
        self.manager.notify(EventType::Message, result.clone());
        self.manager.notify(EventType::Mq, result.clone());
        result
    }
}

fn generate_result(u_id: String) -> LotteryResult {
    LotteryResult::new(u_id, String::from("Congratulation! First Prize!"))
}
