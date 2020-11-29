use super::Matter;

pub struct MarcoPoloTile {}

impl MarcoPoloTile {}

impl Matter for MarcoPoloTile {
    fn scene<'a>(&self) -> &'a str {
        "地板"
    }
    fn brand<'a>(&self) -> &'a str {
        "马可波罗"
    }
    fn model<'a>(&self) -> &'a str {
        "缺省"
    }
    fn desc<'a>(&self) -> &'a str {
        "奥观海"
    }
    fn price(&self) -> f64 {
        2333.0
    }
}
