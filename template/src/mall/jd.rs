use crate::{Client, HashMap, MallTrait};

#[derive(Debug)]
pub struct JdMall {
    u_id: String,
    u_pwd: String,
}

impl MallTrait for JdMall {
    fn new(u_id: String, u_pwd: String) -> Self {
        Self { u_id, u_pwd }
    }

    fn login(&self) -> bool {
        println!("login to the JD as {:?} with {:?}", self.u_id, self.u_pwd);
        true
    }

    fn reptile(&self, url: String) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let urls = vec![url];
        let client = Client {};
        let names = client.get_name(urls);
        println!("JD: {:?}", names);
        map.insert("laptop".into(), "4396".into());
        map.insert("cell phone".into(), "2200".into());
        map
    }
}
