use super::Matter;

pub struct LevelOneCeiling {}

impl LevelOneCeiling {}

impl Matter for LevelOneCeiling {
    fn scene<'a>(&self) -> &'a str {
        "吊顶"
    }
    fn brand<'a>(&self) -> &'a str {
        "装修自带"
    }
    fn model<'a>(&self) -> &'a str {
        "一层吊顶"
    }
    fn desc<'a>(&self) -> &'a str {
        "一层吊顶"
    }
    fn price(&self) -> f64 {
        114514.0
    }
}
