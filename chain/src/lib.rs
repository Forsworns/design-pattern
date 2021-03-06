mod auth_info;
mod auth_link;
mod auth_links;
mod auth_service;

pub use auth_info::*;
pub use auth_link::*;
pub use auth_links::*;
pub use auth_service::*;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::SystemTime;

// the implementation utilizes trait object, which is not the best solution,
// but you can refer to https://forsworns.github.io/zh/blogs/20210120/ for its principle
// moreover, see https://stackoverflow.com/questions/65845197/how-to-define-a-recursive-trait-bound-in-rust
// or https://forsworns.github.io/zh/blogs/20210123/ for a better implementation

#[cfg(test)]
mod tests {
    use crate::*;
    // cargo test -- --nocapture
    #[test]
    fn it_works() {
        let auth_link = Level3AuthLink::new("233".into(), "A".into()).append_next(
            Box::new(Level2AuthLink::new("2333".into(), "B".into()))
                .append_next(Box::new(Level1AuthLink::new("23333".into(), "C".into()))),
        );
        println!(
            "{:?}",
            auth_link.do_auth("123456".into(), "D".into(), SystemTime::now())
        );
        AuthService::auth("233".into(), "A".into());
        println!(
            "{:?}",
            auth_link.do_auth("123456".into(), "D".into(), SystemTime::now())
        );
        AuthService::auth("2333".into(), "B".into());
        println!(
            "{:?}",
            auth_link.do_auth("123456".into(), "D".into(), SystemTime::now())
        );
        AuthService::auth("23333".into(), "C".into());
        println!(
            "{:?}",
            auth_link.do_auth("123456".into(), "D".into(), SystemTime::now())
        );
    }
}
