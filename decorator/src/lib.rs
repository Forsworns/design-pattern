mod decorators;
mod interceptors;

pub use decorators::*;
pub use interceptors::*;

use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

pub static AUTH_MAP: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    Mutex::new({
        let mut m = HashMap::<String, String>::new();
        m.insert("dog".to_owned(), "queryUserInfor".to_owned());
        m.insert("cat".to_owned(), "queryUserInfor".to_owned());
        m
    })
});

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let request = String::from("1successdog");
        let success = LoginSsoDecorator::new(SsoInterceptor {}).pre_handle(
            &request,
            &"烫烫烫".to_owned(),
            empty_handler,
        );
        println!("Request: {}, results: {}", request, success);
    }

    fn empty_handler(_s1: String, _s2: String) -> bool {
        false
    }
}
