mod activity;
mod states;

pub use activity::*;
pub use states::*;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::SystemTime;

#[cfg(test)]
mod tests {
    use crate::*;
    // cargo test -- --nocapture
    #[test]
    fn edit2arraign() {
        let id = "100001";
        ActivityService::init(id, Status::Editing);
        let handler = StateHandler::new();
        let result = handler.arraignment(id, Status::Editing);
        println!("result is: {:?}", result);
        println!("activity into: {:?}", ActivityService::query_activity(id));
    }

    #[test]
    fn edit2open() {
        let id = "100001";
        ActivityService::init(id, Status::Editing);
        let handler = StateHandler::new();
        let result = handler.open(id, Status::Editing);
        println!("result is: {:?}", result);
        println!("activity into: {:?}", ActivityService::query_activity(id));
    }

    #[test]
    fn refuse2do() {
        let id = "100001";
        ActivityService::init(id, Status::Refuse);
        let handler = StateHandler::new();
        let result = handler.doing(id, Status::Refuse);
        println!("result is: {:?}", result);
        println!("activity into: {:?}", ActivityService::query_activity(id));
    }

    #[test]
    fn refuse2revoke() {
        let id = "100001";
        ActivityService::init(id, Status::Refuse);
        let handler = StateHandler::new();
        let result = handler.check_revoke(id, Status::Refuse);
        println!("result is: {:?}", result);
        println!("activity into: {:?}", ActivityService::query_activity(id));
    }
}
