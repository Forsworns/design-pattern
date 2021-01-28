use crate::{ActivityService, Res, State, Status};

pub struct Close {}

impl State for Close {
    fn arraignment(&self, _id: &'static str, _curr: Status) -> Res {
        Ok("closed, cannot arraign")
    }

    fn check_pass(&self, _id: &'static str, _curr: Status) -> Res {
        Err("closed, cannot pass")
    }

    fn check_refuse(&self, _id: &'static str, _curr: Status) -> Res {
        Err("closed, cannot reject")
    }

    fn check_revoke(&self, _id: &'static str, _curr: Status) -> Res {
        Err("closed, cannot revoke")
    }

    fn close(&self, _id: &'static str, _curr: Status) -> Res {
        Ok("closed, canot close aging")
    }

    fn open(&self, id: &'static str, curr: Status) -> Res {
        ActivityService::exec_status(id, curr, Status::Open);
        Ok("not close, cannot open")
    }

    fn doing(&self, _id: &'static str, _curr: Status) -> Res {
        Err("closed, cannot modify")
    }
}
