use crate::LotteryResult;

pub type TypeId = u8;

pub trait EventListener {
    fn do_event(&self, result: LotteryResult);
    fn type_id(&self) -> TypeId;
}

// judge the two listeners are the same type, see `fn unsubscribe() in event_manager.rs`
impl<T: EventListener> PartialEq<T> for dyn EventListener {
    fn eq(&self, other: &T) -> bool {
        self.type_id() == other.type_id()
    }
}

pub struct MessageEventListener {}

impl EventListener for MessageEventListener {
    fn do_event(&self, result: LotteryResult) {
        println!("send message to user: {:?}", result);
    }

    fn type_id(&self) -> TypeId {
        0
    }
}

pub struct MqEventListener {}

impl EventListener for MqEventListener {
    fn do_event(&self, result: LotteryResult) {
        println!("save the result: {:?}", result);
    }

    fn type_id(&self) -> TypeId {
        1
    }
}
