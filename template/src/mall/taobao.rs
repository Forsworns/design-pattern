use crate::{Client, HashMap, MallTrait};

#[derive(Debug)]
pub struct TaobaoMall {
    u_id: String,
    u_pwd: String,
}

impl MallTrait for TaobaoMall {
    fn new(u_id: String, u_pwd: String) -> Self {
        Self { u_id, u_pwd }
    }

    fn login(&self) -> bool {
        println!(
            "login to the TaoBao as {:?} with {:?}",
            self.u_id, self.u_pwd
        );
        true
    }

    fn reptile(&self, url: String) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let urls = vec![url];
        let client = Client {};
        let names = client.get_name(urls);
        println!("Taobao: {:?}", names);
        map.insert("apple".into(), "233".into());
        map.insert("orange".into(), "2333".into());
        map
    }
}
