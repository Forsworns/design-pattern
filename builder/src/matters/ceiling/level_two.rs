use super::Matter;

pub struct LevelTwoCeiling {}

impl LevelTwoCeiling {}

impl Matter for LevelTwoCeiling {
    fn scene<'a>(&self) -> &'a str {
        "吊顶"
    }
    fn brand<'a>(&self) -> &'a str {
        "装修自带"
    }
    fn model<'a>(&self) -> &'a str {
        "二层吊顶"
    }
    fn desc<'a>(&self) -> &'a str {
        "二层吊顶"
    }
    fn price(&self) -> f64 {
        1919810.0
    }
}
