use crate::{ActivityService, Res, State, Status};

pub struct Open {}

impl State for Open {
    fn arraignment(&self, _id: &'static str, _curr: Status) -> Res {
        Err("open, cannot arraign")
    }

    fn check_pass(&self, _id: &'static str, _curr: Status) -> Res {
        Err("open, cannot pass")
    }

    fn check_refuse(&self, _id: &'static str, _curr: Status) -> Res {
        Err("open, cannot reject")
    }

    fn check_revoke(&self, _id: &'static str, _curr: Status) -> Res {
        Err("open, cannot revoke")
    }

    fn close(&self, id: &'static str, curr: Status) -> Res {
        ActivityService::exec_status(id, curr, Status::Close);
        Ok("close, success")
    }

    fn open(&self, _id: &'static str, _curr: Status) -> Res {
        Err("have opented, cannot open again")
    }

    fn doing(&self, id: &'static str, curr: Status) -> Res {
        ActivityService::exec_status(id, curr, Status::Doing);
        Ok("doing, success")
    }
}
