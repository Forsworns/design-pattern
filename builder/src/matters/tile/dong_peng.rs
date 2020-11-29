use super::Matter;

pub struct DongPengTile {}

impl DongPengTile {}

impl Matter for DongPengTile {
    fn scene<'a>(&self) -> &'a str {
        "瓷砖"
    }
    fn brand<'a>(&self) -> &'a str {
        "东鹏"
    }
    fn model<'a>(&self) -> &'a str {
        "10001"
    }
    fn desc<'a>(&self) -> &'a str {
        "东鹏特饮"
    }
    fn price(&self) -> f64 {
        2333.0
    }
}
