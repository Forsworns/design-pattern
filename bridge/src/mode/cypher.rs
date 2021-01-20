use super::IPayMode;

pub struct Cypher{}

impl IPayMode for Cypher{
    fn security(&self, uid: &str)->bool{
        println!("{} cypher", uid);
        true
    }
}