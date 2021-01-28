use crate::{ActivityService, Res, State, Status};

pub struct Pass {}

impl State for Pass {
    fn arraignment(&self, _id: &'static str, _curr: Status) -> Res {
        Err("passed, cannot arraign")
    }

    fn check_pass(&self, _id: &'static str, _curr: Status) -> Res {
        Err("passed, cannot pass")
    }

    fn check_refuse(&self, _id: &'static str, _curr: Status) -> Res {
        Err("passed, cannot reject")
    }

    fn check_revoke(&self, _id: &'static str, _curr: Status) -> Res {
        Err("passed, cannot revoke")
    }

    fn close(&self, id: &'static str, curr: Status) -> Res {
        ActivityService::exec_status(id, curr, Status::Close);
        Ok("close, success")
    }

    fn open(&self, _id: &'static str, _curr: Status) -> Res {
        Err("not close, cannot open")
    }

    fn doing(&self, id: &'static str, curr: Status) -> Res {
        ActivityService::exec_status(id, curr, Status::Doing);
        Ok("doing, success")
    }
}
