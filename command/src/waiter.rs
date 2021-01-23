use crate::CuisineTrait;

pub struct Waiter {
    cuisines: Vec<Box<dyn CuisineTrait>>, // cannot use trait object here ... since you have to
}

impl Waiter {
    pub fn new() -> Self {
        Self {
            cuisines: Vec::new(),
        }
    }

    pub fn order(&mut self, cuisine: Box<dyn CuisineTrait>) {
        self.cuisines.push(cuisine.clone());
    }

    pub fn place_order(&mut self) {
        for cuisine in &self.cuisines {
            cuisine.as_ref().cook();
        }
        self.cuisines.clear();
    }
}
