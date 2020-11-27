use factory::{ServiceType, StoreFactory};

#[test]
fn get_service() {
    StoreFactory::get_commodity_service(ServiceType::Coupon);
}
