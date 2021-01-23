use crate::{CookTrait, CuisineTrait, ShanDongCook};

#[derive(Clone)]
pub struct ShanDongCuisine {
    cook: ShanDongCook,
}

impl ShanDongCuisine {
    pub fn new(cook: ShanDongCook) -> Self {
        Self { cook }
    }
}

impl CuisineTrait for ShanDongCuisine {
    fn cook(&self) {
        self.cook.do_cooking();
    }
}
