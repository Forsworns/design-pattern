use crate::{HashMap, Lazy, Mutex, SystemTime};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Status {
    Editing,
    Check,
    Pass,
    Refuse,
    Doing,
    Close,
    Open,
}

#[derive(Debug)]
pub struct ActivityInfo {
    id: String,
    name: String,
    status: Status,
    begin_time: SystemTime,
    end_time: SystemTime,
}

impl ActivityInfo {
    pub fn new(
        id: String,
        name: String,
        status: Status,
        begin_time: SystemTime,
        end_time: SystemTime,
    ) -> Self {
        Self {
            id,
            name,
            status,
            begin_time,
            end_time,
        }
    }
}

static STATUS_MAP: Lazy<Mutex<HashMap<&str, Status>>> = Lazy::new(|| {
    Mutex::new({
        let m = HashMap::new();
        m
    })
});

pub struct ActivityService {}

impl ActivityService {
    pub fn init(id: &'static str, status: Status) {
        STATUS_MAP.lock().unwrap().insert(id, status);
    }

    pub fn query_activity(id: &str) -> ActivityInfo {
        let activity = ActivityInfo::new(
            id.into(),
            String::from("New activity"),
            *STATUS_MAP.lock().unwrap().get(id).unwrap(),
            SystemTime::now(),
            SystemTime::now(),
        );
        activity
    }

    pub fn query_activity_status(id: &str) -> Status {
        *STATUS_MAP.lock().unwrap().get(id).unwrap()
    }

    pub fn exec_status(id: &'static str, before: Status, after: Status) {
        if before != *STATUS_MAP.lock().unwrap().get(id).unwrap() {
            return;
        }
        STATUS_MAP.lock().unwrap().insert(id, after);
    }
}
