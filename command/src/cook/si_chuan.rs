use crate::CookTrait;
#[derive(Clone)]
pub struct SiChuanCook {}

impl CookTrait for SiChuanCook {
    fn do_cooking(&self) {
        println!("Si Chuan Cook! Tasty!");
    }
}
