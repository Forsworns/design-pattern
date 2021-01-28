use crate::{ActivityService, Res, State, Status};

pub struct Refuse {}

impl State for Refuse {
    fn arraignment(&self, _id: &'static str, _curr: Status) -> Res {
        Err("have refused, cannot check")
    }

    fn check_pass(&self, _id: &'static str, _curr: Status) -> Res {
        Err("have refused, cannot pass")
    }

    fn check_refuse(&self, _id: &'static str, _curr: Status) -> Res {
        Ok("have refused, do not ask again")
    }

    fn check_revoke(&self, _id: &'static str, _curr: Status) -> Res {
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
        Err("refused, cannot modify")
    }
}
