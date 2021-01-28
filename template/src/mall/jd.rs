use crate::{HashMap, MallTrait};

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

    fn reptile(&self, _url: String) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("laptop".into(), "4396".into());
        map.insert("cell phone".into(), "2200".into());
        map
    }
}
