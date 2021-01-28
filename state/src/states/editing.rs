use crate::{ActivityService, Res, State, Status};

pub struct Editing {}

impl State for Editing {
    fn arraignment(&self, id: &'static str, curr: Status) -> Res {
        ActivityService::exec_status(id, curr, Status::Check);
        Ok("arraignment, success")
    }

    fn check_pass(&self, _id: &'static str, _curr: Status) -> Res {
        Err("editing, cannot pass")
    }

    fn check_refuse(&self, _id: &'static str, _curr: Status) -> Res {
        Err("editing, cannot reject")
    }

    fn check_revoke(&self, _id: &'static str, _curr: Status) -> Res {
        Err("editing, cannot revoke")
    }

    fn close(&self, id: &'static str, curr: Status) -> Res {
        ActivityService::exec_status(id, curr, Status::Close);
        Ok("close, success")
    }

    fn open(&self, _id: &'static str, _curr: Status) -> Res {
        Err("not close, cannot open")
    }

    fn doing(&self, _id: &'static str, _curr: Status) -> Res {
        Err("editing, cannot modify")
    }
}
