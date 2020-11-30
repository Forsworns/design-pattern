use super::Question;

#[derive(Clone)]
pub struct EssayQuestion {
    name: String,
    key: String,
}

impl EssayQuestion {
    pub fn new(name: &str, key: &str) -> Self {
        Self {
            name: String::from(name),
            key: String::from(key),
        }
    }
}

impl Question for EssayQuestion {}

impl ToString for EssayQuestion {
    fn to_string(&self) -> String {
        let mut res = String::new();
        res.push_str(format!("问题: {}\n", self.name).as_str());
        res.push_str(format!("答案: {}\n", self.key).as_str());
        res
    }
}
