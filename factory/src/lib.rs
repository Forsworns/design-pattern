use std::collections::HashMap;

mod commodity;
mod instance;

trait Commodity{
    fn send_commodity(&self, uid:String, commodity_id:String, biz_id:String, ext_map:HashMap<String, String>);
}

pub enum ServiceType {
    Coupon,
    Goods,
    Card,
}

pub struct StoreFactory{}

impl StoreFactory {
    fn get_commodity_service(service_type: ServiceType) -> Option<Box<dyn Commodity>>{
        match service_type {
            ServiceType::Coupon => Some(Box::new(commodity::coupon::Service{})),
            ServiceType::Goods => Some(Box::new(commodity::goods::Service{})),
            ServiceType::Card => Some(Box::new(commodity::card::Service{})),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
