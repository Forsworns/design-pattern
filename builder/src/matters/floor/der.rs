use super::Matter;

pub struct DerFloor {}

impl DerFloor {}

impl Matter for DerFloor {
    fn scene<'a>(&self) -> &'a str {
        "地板"
    }
    fn brand<'a>(&self) -> &'a str {
        "德尔"
    }
    fn model<'a>(&self) -> &'a str {
        "A+"
    }
    fn desc<'a>(&self) -> &'a str {
        "火星企业"
    }
    fn price(&self) -> f64 {
        2333.0
    }
}
