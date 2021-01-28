use crate::{HashMap, MallTrait};

#[derive(Debug)]
pub struct DangdangMall {
    u_id: String,
    u_pwd: String,
}

impl MallTrait for DangdangMall {
    fn new(u_id: String, u_pwd: String) -> Self {
        Self { u_id, u_pwd }
    }

    fn login(&self) -> bool {
        println!(
            "login to the DangDang as {:?} with {:?}",
            self.u_id, self.u_pwd
        );
        true
    }

    fn reptile(&self, _url: String) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("SICP".into(), "443".into());
        map.insert("CSAPP".into(), "339".into());
        map
    }
}
