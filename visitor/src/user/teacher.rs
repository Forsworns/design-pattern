use crate::{User, UserEnum, Visitor};

#[derive(Debug, Clone)]
pub struct Teacher {
    pub name: &'static str,
    identity: &'static str,
    class: &'static str,
}

impl User for Teacher {
    fn accept(&self, visitor: &impl Visitor) {
        let teacher = UserEnum::Teacher(self.clone());
        visitor.visit(teacher);
    }
}

impl Teacher {
    pub fn new(name: &'static str, identity: &'static str, class: &'static str) -> Self {
        Self {
            name,
            identity,
            class,
        }
    }

    pub fn entrance_ratio(&self) -> f64 {
        0.8
    }
}
