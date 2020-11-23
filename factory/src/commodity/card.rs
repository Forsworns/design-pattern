use crate::{Commodity, HashMap};
use crate::instance::card::Card;

pub struct Service{}

impl Commodity for Service{
    fn send_commodity(&self, uid:String, commodity_id:String, biz_id:String, ext_map:HashMap<String, String>){

    }
}

