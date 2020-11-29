use super::Matter;

pub struct ShengXiangFloor {}

impl ShengXiangFloor {}

impl Matter for ShengXiangFloor {
    fn scene<'a>(&self) -> &'a str {
        "地板"
    }
    fn brand<'a>(&self) -> &'a str {
        "圣象"
    }
    fn model<'a>(&self) -> &'a str {
        "一级"
    }
    fn desc<'a>(&self) -> &'a str {
        "中国驰名"
    }
    fn price(&self) -> f64 {
        233.0
    }
}
