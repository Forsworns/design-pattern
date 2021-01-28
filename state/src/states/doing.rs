use crate::{ActivityService, Res, State, Status};

pub struct Doing {}

impl State for Doing {
    fn arraignment(&self, _id: &'static str, _curr: Status) -> Res {
        Err("doing, cannot arraign")
    }

    fn check_pass(&self, _id: &'static str, _curr: Status) -> Res {
        Err("doing, cannot pass")
    }

    fn check_refuse(&self, _id: &'static str, _curr: Status) -> Res {
        Err("doing, cannot reject")
    }

    fn check_revoke(&self, _id: &'static str, _curr: Status) -> Res {
        Err("doing, cannot revoke")
    }

    fn close(&self, id: &'static str, curr: Status) -> Res {
        ActivityService::exec_status(id, curr, Status::Close);
        Ok("close, success")
    }

    fn open(&self, _id: &'static str, _curr: Status) -> Res {
        Err("doing, cannot open")
    }

    fn doing(&self, _id: &'static str, _curr: Status) -> Res {
        Err("doing, cannot modify")
    }
}
