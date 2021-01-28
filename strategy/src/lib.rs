mod coupon;

mod context {
    pub use crate::CouponTrait;

    pub struct Context<T> {
        coupon: Box<dyn CouponTrait<T>>,
    }

    impl<T> Context<T> {
        pub fn new(coupon: Box<dyn CouponTrait<T>>) -> Self {
            Self { coupon }
        }

        pub fn amount(&self, info: T, price: f64) -> f64 {
            self.coupon.amount(info, price)
        }
    }
}

pub use context::*;
pub use coupon::*;

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn cut() {
        let ctx = Context::new(Box::new(Cut {}));
        let price = ctx.amount(10.0, 100.0);
        assert_eq!(price, 90.0);
    }

    #[test]
    fn discount() {
        let ctx = Context::new(Box::new(Discount {}));
        let price = ctx.amount(0.8, 100.0);
        assert_eq!(price, 80.0);
    }

    #[test]
    fn fixed() {
        let ctx = Context::new(Box::new(Fixed {}));
        let price = ctx.amount(50.0, 100.0);
        assert_eq!(price, 50.0);
    }

    #[test]
    fn target() {
        let ctx = Context::new(Box::new(Target {}));
        let mut info = HashMap::new();
        info.insert("target", 60.0);
        info.insert("discount", 20.0);
        let price = ctx.amount(info, 100.0);
        assert_eq!(price, 80.0);
    }
}
