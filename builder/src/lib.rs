mod matters;
use matters::*;
mod decoration_package;
use decoration_package::{DecorationGrade, DecorationPackage, PackageTrait};

pub struct Builder {}
impl Builder {
    pub fn europe(&self, area: f64) -> DecorationPackage {
        let mut package = DecorationPackage::new(area, DecorationGrade::Europe);
        package
            .append_ceiling(LevelTwoCeiling {})
            .append_coat(DuluxCoat {})
            .append_floor(ShengXiangFloor {});
        package
    }

    pub fn luxury(&self, area: f64) -> DecorationPackage {
        let mut package = DecorationPackage::new(area, DecorationGrade::Luxury);
        package
            .append_ceiling(LevelTwoCeiling {})
            .append_coat(LiBangCoat {})
            .append_floor(MarcoPoloTile {});
        package
    }

    pub fn modern(&self, area: f64) -> DecorationPackage {
        let mut package = DecorationPackage::new(area, DecorationGrade::Modern);
        package
            .append_ceiling(LevelTwoCeiling {})
            .append_coat(DuluxCoat {})
            .append_floor(ShengXiangFloor {});
        package
    }
}

#[cfg(test)]
#[test]
fn test_ceiling_price() {
    let c = LevelOneCeiling {};
    assert_eq!(c.price(), 114514.0);
}

#[cfg(test)]
#[test]
fn test_ceiling_model() {
    let c = LevelOneCeiling {};
    assert_eq!(c.model(), "一层吊顶");
}
