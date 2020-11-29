use super::Matter;

pub struct DuluxCoat {}

impl DuluxCoat {}

impl Matter for DuluxCoat {
    fn scene<'a>(&self) -> &'a str {
        "涂料"
    }
    fn brand<'a>(&self) -> &'a str {
        "多乐士"
    }
    fn model<'a>(&self) -> &'a str {
        "A+"
    }
    fn desc<'a>(&self) -> &'a str {
        "销量绕地球一圈"
    }
    fn price(&self) -> f64 {
        2333.0
    }
}
