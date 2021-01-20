use super::Pay;
use crate::mode::IPayMode;

pub struct Ali<T:IPayMode>{
    pay_mode: T
}

impl<T:IPayMode> Pay<T> for Ali<T>{
    fn new(pay_mode: T)->Self {
        Self{
            pay_mode
        }
    }

    fn transfer(&self, uid: &str, trade_id: &str, amount: u64){
        println!("Ali pay simulation: uid {}, trade id {}, amount {}!", uid, trade_id, amount);
        let security = self.pay_mode.security(uid);
        println!("Security {}", security);
    }

}