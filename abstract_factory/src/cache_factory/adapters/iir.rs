use crate::cache_factory::instance::Iir as Instance;
use crate::Adapter;

pub struct Iir {
    iir: Instance,
}

impl Iir {
    pub fn new() -> Self {
        Self { iir: Instance {} }
    }
}

impl Adapter for Iir {
    fn get(&self, key: String) -> String {
        self.iir.get(key)
    }
    fn set(&self, key: String, val: String) {
        self.iir.set(key, val);
    }
    fn set_with_time(&self, key: String, val: String, timeout: u32) {
        self.iir.set_expire(key, val, timeout);
    }
    fn del(&self, key: String) {
        self.iir.del(key);
    }
}
