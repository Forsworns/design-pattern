mod dangdang;
mod jd;
mod taobao;

pub use dangdang::*;
pub use jd::*;
pub use taobao::*;

use crate::{encode, HashMap};

pub trait MallTrait {
    fn new(u_id: String, u_pwd: String) -> Self;

    fn login(&self) -> bool;

    fn reptile(&self, url: String) -> HashMap<String, String>;

    fn crate_base64(&self, goods_info: HashMap<String, String>) -> String {
        encode(format!("{:?}", goods_info))
    }

    fn generate_goods_poster(&self, url: String) -> String {
        if !self.login() {
            String::default()
        } else {
            let reptile = self.reptile(url);
            self.crate_base64(reptile)
        }
    }
}
