mod client;
mod mall;

pub use client::*;
pub use mall::*;

use base64::encode;
use futures::future::join_all;
use regex::Regex;
use reqwest;
use std::collections::HashMap;
use tokio;

#[cfg(test)]
mod tests {
    // cargo test -- --nocapture
    use crate::*;

    #[test]
    fn test_jd() {
        let jd = JdMall::new("jd_account".into(), "jd_pwd".into());
        let url = "https://item.jd.com/100008348542.html".into();
        let base64 = jd.generate_goods_poster(url);
        println!("result is {}", base64);
    }

    #[test]
    fn test_tb() {
        let tb = TaobaoMall::new("tb_account".into(), "tb_pwd".into());
        // let url = "https://detail.tmall.com/item.htm?id=636811454410&ali_refid=a3_420841_1006:1181790083:N:EJipSAb3HbB0Cz+g+KTenw==:3d60e25f20b4eae4b5c1cd4c76ed7244&ali_trackid=1_3d60e25f20b4eae4b5c1cd4c76ed7244&spm=a231k.21053755.34973277.2&skuId=4738640831926".into();
        let url = "https://detail.tmall.com/item.htm?id=636811454410".into();
        let base64 = tb.generate_goods_poster(url);
        println!("result is {}", base64);
    }

    #[test]
    fn test_dd() {
        let dd = DangdangMall::new("dd_account".into(), "dd_pwd".into());
        let url = "http://product.dangdang.com/1509704171.html".into();
        let base64 = dd.generate_goods_poster(url);
        println!("result is {}", base64);
    }
}
