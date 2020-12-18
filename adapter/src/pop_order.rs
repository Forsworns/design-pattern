use super::Adapter;

struct Service{

}

impl Service{
    fn is_first_order(&self, _uid:&str)->bool{
        true
    }
}

pub struct ServiceAdapter{
    service: Service
}

impl ServiceAdapter{
    pub fn new()->Self{
        Self{
            service: Service{}
        }
    }
}

impl Adapter for ServiceAdapter{
    fn is_first(&self,uid: &str)->bool{
        self.service.is_first_order(uid)
    }
}