use crate::cache_factory::instance::Egm as Instance;
use crate::Adapter;

pub struct Egm {
    egm: Instance,
}

impl Egm {
    pub fn new() -> Self {
        Self { egm: Instance {} }
    }
}

impl Adapter for Egm {
    fn get(&self, key: String) -> String {
        self.egm.gain(key)
    }
    fn set(&self, key: String, val: String) {
        self.egm.set(key, val);
    }
    fn set_with_time(&self, key: String, val: String, timeout: u32) {
        self.egm.set_ex(key, val, timeout);
    }
    fn del(&self, key: String) {
        self.egm.delete(key);
    }
}
