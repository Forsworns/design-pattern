mod parent;
mod principle;

pub use parent::*;
pub use principle::*;

use crate::UserEnum;

pub trait Visitor {
    fn visit(&self, user: UserEnum);
}
