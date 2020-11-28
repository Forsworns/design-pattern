pub struct Egm {}

impl Egm {
    pub fn gain(&self, _key: String) -> String {
        String::from("")
    }

    pub fn set(&self, _key: String, _val: String) {}

    pub fn set_ex(&self, _key: String, _val: String, _time: u32) {}

    pub fn delete(&self, _key: String) {}
}
