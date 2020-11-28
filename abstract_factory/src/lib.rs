mod cache_factory;
pub use cache_factory::{AdapterFactory, Egm, Iir};

pub trait Adapter {
    fn get(&self, key: String) -> String;
    fn set(&self, key: String, val: String);
    fn set_with_time(&self, key: String, val: String, timeout: u32);
    fn del(&self, key: String);
}

// abstract factory
pub trait Abstract {
    fn create(&self, adapter_type: &str) -> Option<Box<dyn Adapter>>;
}

pub struct AbstractFactory {}

impl AbstractFactory {
    pub fn get_factory(factory_type: &str) -> Option<Box<dyn Abstract>> {
        match factory_type {
            "adapter" => Some(Box::new(AdapterFactory {})),
            "another" => Some(Box::new(AnotherFactory {})),
            _ => None,
        }
    }
}

// another factory integrated into the abstract factory
pub struct AnotherFactory {}

impl AnotherFactory {}

impl Abstract for AnotherFactory {
    fn create(&self, _type: &str) -> Option<Box<dyn Adapter>> {
        None
    }
}
