mod ali;
mod wechat;
use crate::mode::IPayMode;

pub use ali::Ali;
pub use wechat::WeChat;

pub trait Pay<T:IPayMode> {
    fn transfer(&self, uid:&str, trade_id:&str, amount: u64);

    fn new(pay_mode: T)->Self;
}

