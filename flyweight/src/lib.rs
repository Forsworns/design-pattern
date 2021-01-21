mod activity;
mod simulator;
mod stock;

pub use activity::*;
pub use simulator::*;
pub use stock::*;

use std::collections::HashMap;
use std::sync::{atomic::AtomicU64, atomic::Ordering};
use std::thread;
use std::time::{Duration, SystemTime};

#[cfg(test)]
mod tests {
    use crate::*;
    // cargo test -- --nocapture

    #[test]
    fn it_works() {
        let simulator: Redis = Redis::new();
        let mut activity_map: HashMap<Id, Activity> = HashMap::new();
        for i in 1..100 {
            let req = 10001u64;
            let activity =
                ActivityController::query_activity_info(req + i / 4, &simulator, &mut activity_map);
            println!("\n\n Request Id: {}\nActivity: {:?}", req + i / 4, activity);
            thread::sleep(Duration::new(1, 0));
        }
    }
}
