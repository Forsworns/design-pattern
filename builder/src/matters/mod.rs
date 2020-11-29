mod ceiling;
mod coat;
mod floor;
mod tile;

pub use ceiling::*;
pub use coat::*;
pub use floor::*;
pub use tile::*;

pub trait Matter {
    fn scene<'a>(&self) -> &'a str;
    fn brand<'a>(&self) -> &'a str;
    fn model<'a>(&self) -> &'a str;
    fn desc<'a>(&self) -> &'a str;
    fn price(&self) -> f64;
}
