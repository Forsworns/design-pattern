use crate::{Client, HashMap, MallTrait};

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

    fn reptile(&self, url: String) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let urls = vec![url];
        let client = Client {};
        let names = client.get_name(urls);
        println!("DangDang: {:?}", names);
        map.insert("SICP".into(), "443".into());
        map.insert("CSAPP".into(), "339".into());
        map
    }
}
