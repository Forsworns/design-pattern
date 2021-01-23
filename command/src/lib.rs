mod cook;
mod cuisine;
mod waiter;

pub use cook::*;
pub use cuisine::*;
pub use waiter::*;

#[cfg(test)]
mod tests {
    use crate::*;
    // cargo test -- --nocapture
    #[test]
    fn it_works() {
        let gd = GuangDongCuisine::new(GuangDongCook {});
        let sc = SiChuanCuisine::new(SiChuanCook {});
        let js = JiangSuCuisine::new(JiangSuCook {});
        let sd = ShanDongCuisine::new(ShanDongCook {});
        let mut waiter = Waiter::new();
        waiter.order(Box::new(gd));
        waiter.order(Box::new(sc));
        waiter.order(Box::new(js));
        waiter.order(Box::new(sd));

        waiter.place_order();
    }
}
