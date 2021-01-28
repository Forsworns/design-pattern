mod cut;
mod discount;
mod fixed;
mod target;

pub use cut::*; // 直降
pub use discount::*; // 折扣
pub use fixed::*; // n元购
pub use target::*; // 满减

pub trait CouponTrait<T> {
    fn amount(&self, info: T, price: f64) -> f64;
}
