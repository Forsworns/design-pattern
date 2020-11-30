use super::{thread_rng, Question, SliceRandom};

pub struct QuestionBank {
    questions: Vec<Box<dyn Question>>,
    pub name: String,
    pub stu_id: String,
}

impl QuestionBank {
    pub fn new() -> Self {
        Self {
            questions: Vec::new(),
            name: String::new(),
            stu_id: String::new(),
        }
    }

    pub fn append(&mut self, q: impl Question + 'static) -> &mut Self {
        self.questions.push(Box::new(q));
        self
    }
}

impl ToString for QuestionBank {
    fn to_string(&self) -> String {
        let mut res = String::new();
        res.push_str(format!("name: {}", self.name).as_str());
        res.push_str(format!("student_id: {}\n\n", self.stu_id).as_str());

        res += self
            .questions
            .iter()
            .fold(String::new(), |acc, q| {
                acc + format!("{}\n", q.as_ref().to_string()).as_str()
            })
            .as_str();

        res
    }
}

impl Clone for QuestionBank {
    fn clone(&self) -> Self {
        let mut questions: Vec<Box<dyn Question>> = Vec::new();
        for q in &self.questions {
            questions.push(q.clone());
        }
        questions.shuffle(&mut thread_rng());
        Self {
            questions,
            name: self.name.clone(),
            stu_id: self.stu_id.clone(),
        }
    }
}
