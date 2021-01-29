use crate::{UserEnum, Visitor};

pub struct Principle {}

impl Visitor for Principle {
    fn visit(&self, user: UserEnum) {
        match user {
            UserEnum::Student(student) => {
                println!("visit student from principle visitor: {:?}", student);
            }
            UserEnum::Teacher(teacher) => {
                println!("visit teacher from principle visitor: {:?}", teacher);
            }
        }
    }
}
