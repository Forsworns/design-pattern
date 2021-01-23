use crate::{CookTrait, CuisineTrait, GuangDongCook};
#[derive(Clone)]
pub struct GuangDongCuisine {
    cook: GuangDongCook,
}

impl GuangDongCuisine {
    pub fn new(cook: GuangDongCook) -> Self {
        Self { cook }
    }
}

impl CuisineTrait for GuangDongCuisine {
    fn cook(&self) {
        self.cook.do_cooking();
    }
}
