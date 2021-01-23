use crate::CookTrait;
#[derive(Clone)]
pub struct ShanDongCook {}

impl CookTrait for ShanDongCook {
    fn do_cooking(&self) {
        println!("Shan Dong Cook");
    }
}
