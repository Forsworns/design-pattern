mod instance;

mod adapters;
pub use adapters::{Egm, Iir};

pub use crate::Adapter;

pub struct AdapterFactory {}

impl AdapterFactory {}

use super::Abstract;
impl Abstract for AdapterFactory {
    fn create(&self, adapter_type: &str) -> Option<Box<dyn Adapter>> {
        match adapter_type {
            "EGM" => Some(Box::new(Egm::new())),
            "IIR" => Some(Box::new(Iir::new())),
            _ => None,
        }
    }
}
