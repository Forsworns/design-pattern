use super::Question;
use crate::{thread_rng, HashMap, SliceRandom};

pub struct ChoiceQuestion {
    name: String,
    option: HashMap<String, String>,
    key: String,
}

impl ChoiceQuestion {
    pub fn new(name: &str, option: HashMap<String, String>, key: &str) -> Self {
        Self {
            option,
            name: String::from(name),
            key: String::from(key),
        }
    }
}

impl Question for ChoiceQuestion {}

impl ToString for ChoiceQuestion {
    fn to_string(&self) -> String {
        let mut res = String::new();
        res.push_str(format!("问题: {}\n", self.name).as_str());
        res.push_str(format!("答案: {}\n", self.key).as_str());
        for (k, v) in &self.option {
            res.push_str(format!("{}. {}\n", k, v).as_str());
        }
        res
    }
}

impl Clone for ChoiceQuestion {
    fn clone(&self) -> Self {
        let mut option = self.option.clone();
        let mut choice_vec: Vec<String> = Vec::new();
        let mut content_vec: Vec<String> = Vec::new();
        for (k, v) in &self.option {
            choice_vec.push(k.clone());
            content_vec.push(v.clone());
        }
        content_vec.shuffle(&mut thread_rng());
        for i in 0..content_vec.len() {
            option.insert(choice_vec[i].clone(), content_vec[i].clone());
        }
        let mut key = String::new();
        for (k, v) in &option {
            if *v == self.key {
                key = k.clone();
                break;
            }
        }
        Self {
            name: self.name.clone(),
            key: key,
            option: option,
        }
    }
}
