use super::Adapter;

struct Service{

}

impl Service{
    fn query_user_order_count(&self, _uid:&str)->u32{
        1
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
        self.service.query_user_order_count(uid) <= 1u32
    }
}