use crate::{HashMap, Lazy, Mutex, SystemTime};

pub static AUTH_MAP: Lazy<Mutex<HashMap<String, SystemTime>>> = Lazy::new(|| {
    Mutex::new({
        let m = HashMap::new();
        m
    })
});

pub struct AuthService {}

impl AuthService {
    pub fn query_auth_info(u_id: &String, order_id: &String) -> Option<SystemTime> {
        let mut query_str = u_id.clone();
        query_str.push_str(order_id);
        if let Some(time) = AUTH_MAP.lock().unwrap().get(&query_str) {
            Some(time.clone())
        } else {
            None
        }
    }

    pub fn auth(mut u_id: String, order_id: String) {
        u_id.push_str(&order_id);
        AUTH_MAP.lock().unwrap().insert(u_id, SystemTime::now());
    }
}
