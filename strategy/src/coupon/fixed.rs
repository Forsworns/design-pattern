use crate::CouponTrait;

pub struct Fixed {}

impl CouponTrait<f64> for Fixed {
    fn amount(&self, info: f64, _price: f64) -> f64 {
        info
    }
}
