use crate::CookTrait;
#[derive(Clone)]
pub struct GuangDongCook {}

impl CookTrait for GuangDongCook {
    fn do_cooking(&self) {
        println!("Guang Dong Cook");
    }
}
