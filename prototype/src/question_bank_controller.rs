use super::{ChoiceQuestion, EssayQuestion, QuestionBank};
use crate::HashMap;

pub struct QuestionBankController {
    qb: QuestionBank,
}

impl QuestionBankController {
    pub fn new() -> Self {
        let mut qb = QuestionBank::new();

        let mut c1 = HashMap::new();
        c1.insert(String::from("A"), String::from("rust"));
        c1.insert(String::from("B"), String::from("c++"));
        c1.insert(String::from("C"), String::from("both"));
        c1.insert(String::from("D"), String::from("none"));

        let mut c2 = HashMap::new();
        c2.insert(String::from("A"), String::from("typescript"));
        c2.insert(String::from("B"), String::from("javascript"));
        c2.insert(String::from("C"), String::from("both"));
        c2.insert(String::from("D"), String::from("none"));

        qb.append(ChoiceQuestion::new("rust/c++?", c1, "none"))
            .append(ChoiceQuestion::new("typescript/javascript?", c2, "none"))
            .append(EssayQuestion::new("advantage?", "safe"));

        Self { qb }
    }

    pub fn create_paper(&mut self, name: &str, stu_id: &str) -> String {
        // here we suppose that, compare with the `clone()`,
        // directly constructing a new instance takes more time (request data from remote server, etc)
        let mut qb = self.qb.clone();
        qb.name = String::from(name);
        qb.stu_id = String::from(stu_id);
        qb.to_string()
    }
}
