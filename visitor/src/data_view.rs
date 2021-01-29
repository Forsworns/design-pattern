use crate::{Student, Teacher, User, UserEnum, Visitor};

pub struct DataView {
    user_lists: Vec<UserEnum>,
}

impl DataView {
    pub fn new() -> Self {
        let user_lists = vec![
            UserEnum::Student(Student::new("a", "1", "A")),
            UserEnum::Student(Student::new("b", "2", "A")),
            UserEnum::Teacher(Teacher::new("c", "3", "A")),
            UserEnum::Teacher(Teacher::new("d", "4", "B")),
        ];
        Self { user_lists }
    }

    pub fn show(&self, visitor: impl Visitor) {
        for user in &self.user_lists {
            // utilize enum to apply overload/specilization, when the trait object is enumerable
            match user {
                UserEnum::Student(s) => {
                    s.accept(&visitor);
                }
                UserEnum::Teacher(t) => {
                    t.accept(&visitor);
                }
            }
        }
    }
}
