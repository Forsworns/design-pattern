use crate::{User, UserEnum, Visitor};

#[derive(Debug, Clone)]
pub struct Student {
    pub name: &'static str,
    identity: &'static str,
    class: &'static str,
}

impl User for Student {
    fn accept(&self, visitor: &impl Visitor) {
        let student = UserEnum::Student(self.clone());
        visitor.visit(student);
    }
}

impl Student {
    pub fn new(name: &'static str, identity: &'static str, class: &'static str) -> Self {
        Self {
            name,
            identity,
            class,
        }
    }

    pub fn ranking(&self) -> u32 {
        120
    }
}
