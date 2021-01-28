mod mall;

pub use mall::*;

use base64::encode;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    // cargo test -- --nocapture
    use crate::*;

    #[test]
    fn test_jd() {
        let jd = JdMall::new("jd_account".into(), "jd_pwd".into());
        let base64 = jd.generate_goods_poster("jd.com".into());
        println!("result is {}", base64);
    }

    #[test]
    fn test_tb() {
        let tb = TaobaoMall::new("tb_account".into(), "tb_pwd".into());
        let base64 = tb.generate_goods_poster("tmall.com".into());
        println!("result is {}", base64);
    }

    #[test]
    fn test_dd() {
        let dd = DangdangMall::new("dd_account".into(), "dd_pwd".into());
        let base64 = dd.generate_goods_poster("dangdang.com".into());
        println!("result is {}", base64);
    }
}
