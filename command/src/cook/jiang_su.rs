use crate::CookTrait;
#[derive(Clone)]
pub struct JiangSuCook {}

impl CookTrait for JiangSuCook {
    fn do_cooking(&self) {
        println!("Jiang Su Cook");
    }
}
