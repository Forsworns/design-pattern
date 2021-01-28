use crate::CouponTrait;

pub struct Cut {}

impl CouponTrait<f64> for Cut {
    fn amount(&self, info: f64, price: f64) -> f64 {
        let price = price - info;
        if price < 1.0 {
            return 1.0;
        } else {
            return price;
        }
    }
}
