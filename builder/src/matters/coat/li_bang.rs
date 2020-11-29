use super::Matter;

pub struct LiBangCoat {}

impl LiBangCoat {}

impl Matter for LiBangCoat {
    fn scene<'a>(&self) -> &'a str {
        "涂料"
    }
    fn brand<'a>(&self) -> &'a str {
        "立邦"
    }
    fn model<'a>(&self) -> &'a str {
        "默认"
    }
    fn desc<'a>(&self) -> &'a str {
        "绿色！"
    }
    fn price(&self) -> f64 {
        233.0
    }
}
