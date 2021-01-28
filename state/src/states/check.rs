use crate::{ActivityService, Res, State, Status};

pub struct Check {}

impl State for Check {
    fn arraignment(&self, _id: &'static str, _curr: Status) -> Res {
        Err("wait for check")
    }

    fn check_pass(&self, id: &'static str, curr: Status) -> Res {
        ActivityService::exec_status(id, curr, Status::Pass);
        Ok("passed, success")
    }

    fn check_refuse(&self, id: &'static str, curr: Status) -> Res {
        ActivityService::exec_status(id, curr, Status::Refuse);
        Ok("refused, success")
    }

    fn check_revoke(&self, id: &'static str, curr: Status) -> Res {
        ActivityService::exec_status(id, curr, Status::Editing);
        Ok("revoked, success")
    }

    fn close(&self, id: &'static str, curr: Status) -> Res {
        ActivityService::exec_status(id, curr, Status::Close);
        Ok("close, success")
    }

    fn open(&self, _id: &'static str, _curr: Status) -> Res {
        Err("not close, cannot open")
    }

    fn doing(&self, _id: &'static str, _curr: Status) -> Res {
        Err("wait for check, cannot modify")
    }
}
