use crate::SystemTime;

#[derive(Clone, Debug, Default)]
pub struct ConfigFile {
    pub version_no: String,
    content: String,
    time: MySystemTime,
    operator: String,
}

impl ConfigFile {
    pub fn new(version_no: &str, content: &str, time: SystemTime, operator: &str) -> Self {
        Self {
            version_no: version_no.to_owned(),
            content: content.to_owned(),
            operator: operator.to_owned(),
            time: MySystemTime { time },
        }
    }
}

#[derive(Clone, Debug)]
struct MySystemTime {
    time: SystemTime,
}

impl Default for MySystemTime {
    fn default() -> Self {
        Self {
            time: SystemTime::now(),
        }
    }
}
