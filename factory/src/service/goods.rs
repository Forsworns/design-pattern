use crate::instance::goods::{DeliverReq, Goods};
use crate::{Commodity, HashMap};

#[derive(Debug, Copy, Clone)]
pub struct Service {
    goods: Goods,
}

impl Service {
    pub fn new() -> Self {
        Self { goods: Goods {} }
    }
}

impl Commodity for Service {
    fn send_commodity(
        &self,
        uid: String,
        commodity_id: String,
        biz_id: String,
        ext_map: HashMap<String, String>,
    ) {
        let consignee_name = ext_map
            .get("consignee_name")
            .unwrap_or(&String::from("Kyrie"))
            .clone();
        let consignee_mobile = ext_map
            .get("consignee_mobile")
            .unwrap_or(&String::from("110"))
            .clone();
        let consignee_address = ext_map
            .get("consignee_address")
            .unwrap_or(&String::from("github"))
            .clone();
        let req = DeliverReq::new(
            uid,
            commodity_id,
            biz_id,
            consignee_name,
            consignee_mobile,
            consignee_address,
        );
        match self.goods.deliver(req) {
            Ok("0000") => {
                println!("Goods are delivered!");
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
