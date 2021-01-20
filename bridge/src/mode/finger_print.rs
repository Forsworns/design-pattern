use super::IPayMode;

pub struct FingerPrint{}

impl IPayMode for FingerPrint{
    fn security(&self, uid: &str)->bool{
        println!("{} finger print", uid);
        true
    }
}