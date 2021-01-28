use crate::{CouponTrait, HashMap};

pub struct Target {}

impl CouponTrait<HashMap<&str, f64>> for Target {
    fn amount(&self, info: HashMap<&str, f64>, price: f64) -> f64 {
        let target = *info.get("target").unwrap();
        let discount = *info.get("discount").unwrap();
        if price < target {
            return price;
        }
        let price = price - discount;
        if price < 1.0 {
            return 1.0;
        } else {
            return price;
        }
    }
}
