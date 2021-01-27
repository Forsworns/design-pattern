mod event_manager;
mod listener;
mod lottery;

pub use event_manager::*;
pub use listener::*;
pub use lottery::*;

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::*;
    // cargo test -- --nocapture
    #[test]
    fn it_works() {
        let mut service = LotteryService::new();
        let result = service.draw("1145141919810".to_owned());
        println!("result: {:?}", result);
    }
}
