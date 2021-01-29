use crate::{UserEnum, Visitor};

pub struct Parent {}

impl Visitor for Parent {
    fn visit(&self, user: UserEnum) {
        match user {
            UserEnum::Student(student) => {
                println!(
                    "visit student from parent visitor: Student {}, ranking {}",
                    student.name,
                    student.ranking()
                );
            }
            UserEnum::Teacher(teacher) => {
                println!(
                    "visit teacher from parent visitor: Teacher {}, entrance ratio {}",
                    teacher.name,
                    teacher.entrance_ratio()
                );
            }
        }
    }
}
