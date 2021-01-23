use crate::{CookTrait, CuisineTrait, SiChuanCook};

#[derive(Clone)]
pub struct SiChuanCuisine {
    cook: SiChuanCook,
}

impl SiChuanCuisine {
    pub fn new(cook: SiChuanCook) -> Self {
        Self { cook }
    }
}

impl CuisineTrait for SiChuanCuisine {
    fn cook(&self) {
        self.cook.do_cooking();
    }
}
