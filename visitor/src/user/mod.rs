mod student;
mod teacher;

pub use student::*;
pub use teacher::*;

use crate::Visitor;

pub trait User {
    fn accept(&self, visitor: &impl Visitor);
}

#[derive(Clone)]
pub enum UserEnum {
    Student(Student),
    Teacher(Teacher),
}
