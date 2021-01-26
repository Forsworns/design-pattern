#[allow(dead_code)]
mod group;

use std::collections::HashMap;

pub use group::*;

#[cfg(test)]
mod tests {
    use crate::*;
    // cargo test -- --nocapture
    #[test]
    fn it_works() {
        let mut group = Group::new("1".into(), "big brother".into());
        println!("group done");
        group.add(Employee::new("2", "cock", "second"));
        group.add(Employee::new("3", "dog", "second"));
        group.add(Employee::new("4", "cat", "third"));
        group.add(Employee::new("5", "monkey", "third"));
        group.add(Employee::new("6", "pig", "fourth"));
        group.add(Employee::new("7", "mouse", "fourth"));
        group.add(Employee::new("8", "kangaroo", "fourth"));

        group.add_link("1", Link::new("1", "2"));
        group.add_link("1", Link::new("1", "3"));
        group.add_link("2", Link::new("2", "4"));
        group.add_link("2", Link::new("2", "5"));
        group.add_link("5", Link::new("5", "6"));
        group.add_link("5", Link::new("5", "7"));
        group.add_link("5", Link::new("5", "8"));

        for e in group {
            // once trait Iterator is implemented, the IntoIterator is automatically implemented
            println!("{:?}", e);
        }
    }
}
