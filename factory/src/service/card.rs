use crate::instance::card::Card;
use crate::utils::*;
use crate::{Commodity, HashMap};

pub struct Service {
    card: Card,
}

impl Service {
    pub fn new() -> Self {
        Self { card: Card {} }
    }
}

impl Commodity for Service {
    fn send_commodity(
        &self,
        uid: String,
        _commodity_id: String,
        biz_id: String,
        _ext_map: HashMap<String, String>,
    ) {
        let mobile = query_mobile(uid);
        match self.card.grant_token(mobile, biz_id) {
            Ok("0000") => {
                println!("Card granted!");
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
