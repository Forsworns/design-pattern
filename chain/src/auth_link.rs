use crate::{AuthInfo, SystemTime};
pub trait AuthLinkTrait: AuthCloneTrait {
    fn do_auth(&self, u_id: String, order_id: String, auth_date: SystemTime) -> AuthInfo;
    fn next(&self) -> Option<Box<dyn AuthLinkTrait>>;
}

pub trait AuthAppendTrait {
    fn append_next(&mut self, next: Box<dyn AuthLinkTrait>) -> Box<dyn AuthLinkTrait>;
}
pub trait AuthCloneTrait {
    fn clone_box(&self) -> Box<dyn AuthLinkTrait>;
}

impl<T> AuthCloneTrait for T
where
    T: 'static + AuthLinkTrait + AuthAppendTrait + Clone,
{
    fn clone_box(&self) -> Box<dyn AuthLinkTrait> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn AuthLinkTrait> {
    fn clone(&self) -> Box<dyn AuthLinkTrait> {
        self.clone_box()
    }
}
