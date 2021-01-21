use crate::{HashMap, Redis, Stock, SystemTime};

pub type Id = u64;

#[derive(Clone, Debug)]
pub struct Activity {
    id: u64,
    name: String,
    desc: String,
    start_time: SystemTime,
    end_time: SystemTime,
    stock: Stock,
}

impl Activity {
    pub fn set_stock(&mut self, stock: Stock) {
        self.stock = stock;
    }
}

pub struct ActivityFactory {}

impl ActivityFactory {
    fn get_activity(id: Id, activity_map: &mut HashMap<Id, Activity>) -> Activity {
        if let Some(activity) = activity_map.get(&id) {
            return activity.clone();
        } else {
            let activity = Activity {
                id,
                name: "book".into(),
                desc: "book in sale".into(),
                start_time: SystemTime::now(),
                end_time: SystemTime::now(),
                stock: Stock {
                    total: 100000000u64,
                    used: 0,
                },
            };
            activity_map.insert(id, activity.clone());
            return activity;
        }
    }
}

pub struct ActivityController {}

impl ActivityController {
    pub fn query_activity_info(
        id: Id,
        redis: &Redis,
        activity_map: &mut HashMap<Id, Activity>,
    ) -> Activity {
        let mut activity = ActivityFactory::get_activity(id, activity_map);
        let stock = Stock {
            total: 100000000u64,
            used: redis.get_stock_used(id),
        };
        activity.set_stock(stock);
        activity
    }
}
