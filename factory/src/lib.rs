use std::collections::HashMap;

mod instance;
mod service;

pub trait Commodity {
    fn send_commodity(
        &self,
        uid: String,
        commodity_id: String,
        biz_id: String,
        ext_map: HashMap<String, String>,
    );
}

pub enum ServiceType {
    Coupon,
    Goods,
    Card,
}

pub struct StoreFactory {}

impl StoreFactory {
    pub fn get_commodity_service(service_type: ServiceType) -> Option<Box<dyn Commodity>> {
        match service_type {
            ServiceType::Coupon => Some(Box::new(service::coupon::Service::new())),
            ServiceType::Goods => Some(Box::new(service::goods::Service::new())),
            ServiceType::Card => Some(Box::new(service::card::Service::new())),
        }
    }
}

mod utils {
    pub fn query_mobile(uid: String) -> String {
        uid
    }
}
