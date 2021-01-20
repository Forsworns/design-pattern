mod cypher;
mod face;
mod finger_print;

pub use cypher::Cypher;
pub use face::Face;
pub use finger_print::FingerPrint;

pub trait IPayMode{
    fn security(&self, uid:&str)->bool;
}