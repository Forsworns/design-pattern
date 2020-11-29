use super::matters::Matter;

#[derive(Debug)]
pub enum DecorationGrade {
    Modern,
    Europe,
    Luxury,
}

pub trait PackageTrait {
    fn append_ceiling(&mut self, m: impl Matter + 'static) -> &mut Self;
    fn append_coat(&mut self, m: impl Matter + 'static) -> &mut Self;
    fn append_tile(&mut self, m: impl Matter + 'static) -> &mut Self;
    fn append_floor(&mut self, m: impl Matter + 'static) -> &mut Self;
}

pub struct DecorationPackage {
    list: Vec<Box<dyn Matter>>,
    price: f64,
    area: f64,
    grade: DecorationGrade,
}

impl DecorationPackage {
    pub fn new(area: f64, grade: DecorationGrade) -> Self {
        Self {
            area,
            grade,
            price: 0.0,
            list: Vec::new(),
        }
    }

    pub fn get_detail(&self) -> String {
        let mut detail = String::from("------------------------\n");
        detail.push_str(format!("等级：{:?}\n", self.grade).as_str());
        detail.push_str(format!("面积：{}\n", self.area).as_str());
        detail.push_str(format!("价格：{}\n", self.price).as_str());
        detail.push_str("清单：");
        for item in &self.list {
            detail.push_str(
                format!(
                    "{}，{}，{}，平米价格：{} 元\n",
                    item.scene(),
                    item.model(),
                    item.brand(),
                    item.price()
                )
                .as_str(),
            )
        }
        detail
    }
}

impl PackageTrait for DecorationPackage {
    // all of the objects of type T:Matter are moved and borrowed
    fn append_ceiling(&mut self, m: impl Matter + 'static) -> &mut Self {
        // cannot change the operation order of the self.price and self.list
        self.price += 0.2 * self.area * m.price();
        self.list.push(Box::new(m));
        self
    }

    fn append_coat(&mut self, m: impl Matter + 'static) -> &mut Self {
        self.price += 1.4 * self.area * m.price();
        self.list.push(Box::new(m));
        self
    }

    fn append_floor(&mut self, m: impl Matter + 'static) -> &mut Self {
        self.price += self.area * m.price();
        self.list.push(Box::new(m));
        self
    }

    fn append_tile(&mut self, m: impl Matter + 'static) -> &mut Self {
        self.price += self.area * m.price();
        self.list.push(Box::new(m));
        self
    }
}
