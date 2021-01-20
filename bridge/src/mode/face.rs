use super::IPayMode;

pub struct Face{}

impl IPayMode for Face{
    fn security(&self, uid: &str)->bool{
        println!("{} face", uid);
        true
    }
}