mod guang_dong;
mod jiang_su;
mod shan_dong;
mod si_chuan;

pub use guang_dong::*;
pub use jiang_su::*;
pub use shan_dong::*;
pub use si_chuan::*;

pub trait CookTrait {
    fn do_cooking(&self);
}
