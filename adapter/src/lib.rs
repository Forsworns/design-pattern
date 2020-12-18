mod pop_order;
mod inside_order;

pub use pop_order::ServiceAdapter as PopAdapter;
pub use inside_order::ServiceAdapter as InsideAdapter;

pub trait Adapter{
    fn is_first(&self, uid:&str)->bool;
}

#[cfg(test)]
mod tests{
    use crate::{PopAdapter, InsideAdapter, Adapter};

    #[test]
    fn adapter() {
        let pop_order_adapter = PopAdapter::new();
        let inside_order_adapter = InsideAdapter::new();
        println!("check whether it is the first inside order: {}",inside_order_adapter.is_first("10001"));
        println!("check whether it is the first pop order: {}", pop_order_adapter.is_first("10001"));
    }
}