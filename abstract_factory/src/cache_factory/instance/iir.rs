pub struct Iir {}

impl Iir {
    pub fn get(&self, _key: String) -> String {
        String::from("")
    }

    pub fn set(&self, _key: String, _val: String) {}

    pub fn set_expire(&self, _key: String, _val: String, _time: u32) {}

    pub fn del(&self, _key: String) {}
}
