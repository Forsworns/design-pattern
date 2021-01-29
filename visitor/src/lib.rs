mod data_view;
mod user;
mod visitor;

pub use self::visitor::*; // ambiguous with the crate visitor
pub use data_view::*;
pub use user::*;

#[cfg(test)]
mod tests {
    use crate::*;
    // cargo test -- --nocapture
    #[test]
    fn it_works() {
        let data_view = DataView::new();
        println!("view from parent");
        data_view.show(Parent {});
        println!("view from principle");
        data_view.show(Principle {});
    }
}
