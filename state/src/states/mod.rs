mod check;
mod close;
mod doing;
mod editing;
mod open;
mod pass;
mod refuse;

pub use check::*;
pub use close::*;
pub use doing::*;
pub use editing::*;
pub use open::*;
pub use pass::*;
pub use refuse::*;

use crate::{HashMap, Status};

pub type Res = Result<&'static str, &'static str>;

pub trait State {
    fn arraignment(&self, id: &'static str, curr: Status) -> Res;
    fn check_pass(&self, id: &'static str, curr: Status) -> Res;
    fn check_refuse(&self, id: &'static str, curr: Status) -> Res;
    fn check_revoke(&self, id: &'static str, curr: Status) -> Res;
    fn close(&self, id: &'static str, curr: Status) -> Res;
    fn open(&self, id: &'static str, curr: Status) -> Res;
    fn doing(&self, id: &'static str, curr: Status) -> Res;
}

pub struct StateHandler {
    state_map: HashMap<Status, Box<dyn State>>,
}

impl StateHandler {
    pub fn new() -> Self {
        let mut state_map = HashMap::<Status, Box<dyn State>>::new();
        state_map.insert(Status::Check, Box::new(Check {}));
        state_map.insert(Status::Doing, Box::new(Doing {}));
        state_map.insert(Status::Editing, Box::new(Editing {}));
        state_map.insert(Status::Open, Box::new(Open {}));
        state_map.insert(Status::Pass, Box::new(Pass {}));
        state_map.insert(Status::Refuse, Box::new(Refuse {}));
        Self { state_map }
    }

    pub fn arraignment(&self, id: &'static str, curr: Status) -> Res {
        self.state_map.get(&curr).unwrap().arraignment(id, curr)
    }

    pub fn check_pass(&self, id: &'static str, curr: Status) -> Res {
        self.state_map.get(&curr).unwrap().check_pass(id, curr)
    }

    pub fn check_refuse(&self, id: &'static str, curr: Status) -> Res {
        self.state_map.get(&curr).unwrap().check_refuse(id, curr)
    }

    pub fn check_revoke(&self, id: &'static str, curr: Status) -> Res {
        self.state_map.get(&curr).unwrap().check_revoke(id, curr)
    }

    pub fn close(&self, id: &'static str, curr: Status) -> Res {
        self.state_map.get(&curr).unwrap().close(id, curr)
    }

    pub fn open(&self, id: &'static str, curr: Status) -> Res {
        self.state_map.get(&curr).unwrap().open(id, curr)
    }

    pub fn doing(&self, id: &'static str, curr: Status) -> Res {
        self.state_map.get(&curr).unwrap().doing(id, curr)
    }
}
