use crate::instance::coupon::Coupon;
use crate::{Commodity, HashMap};

#[derive(Debug, Copy, Clone)]
pub struct Service {
    coupon: Coupon,
}

impl Service {
    pub fn new() -> Self {
        Self { coupon: Coupon {} }
    }
}

impl Commodity for Service {
    fn send_commodity(
        &self,
        uid: String,
        commodity_id: String,
        biz_id: String,
        _ext_map: HashMap<String, String>,
    ) {
        match self.coupon.send(uid, commodity_id, biz_id) {
            Ok("0000") => {
                println!("Coupon is sent!");
            }
            Ok(_) => {
                println!("Unexpected status code!");
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
