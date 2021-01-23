use crate::{CookTrait, CuisineTrait, JiangSuCook};
#[derive(Clone)]
pub struct JiangSuCuisine {
    cook: JiangSuCook,
}

impl JiangSuCuisine {
    pub fn new(cook: JiangSuCook) -> Self {
        Self { cook }
    }
}

impl CuisineTrait for JiangSuCuisine {
    fn cook(&self) {
        self.cook.do_cooking();
    }
}
