mod age;
mod gender;

use crate::{HashMap, Lazy, Mutex, Rule, TreeNodeLink};

pub use age::UserAgeFilter;
pub use gender::UserGenderFilter;

pub static LOGIC_FILTER_MAP: Lazy<Mutex<HashMap<String, Box<dyn LogicFilter>>>> = Lazy::new(|| {
    Mutex::new({
        let mut m = HashMap::<String, Box<dyn LogicFilter>>::new();
        m.insert("gender".to_owned(), Box::new(UserGenderFilter {}));
        m.insert("age".to_owned(), Box::new(UserAgeFilter {}));
        m
    })
});

pub trait LogicFilter: Send + std::fmt::Debug {
    fn filter(&self, matter_value: &String, info_list: &Vec<TreeNodeLink>) -> i64 {
        for node_link in info_list {
            if self.decision_logic(&matter_value, &node_link) {
                return node_link.get_node_id_to();
            }
        }
        0i64
    }

    fn decision_logic(&self, matter_value: &String, node_link: &TreeNodeLink) -> bool {
        match node_link.get_rule_limit_type() {
            Rule::Eq => *matter_value == node_link.get_rule_limit_value(),
            Rule::G => {
                matter_value.parse::<f64>().unwrap()
                    > node_link.get_rule_limit_value().parse::<f64>().unwrap()
            }
            Rule::L => {
                matter_value.parse::<f64>().unwrap()
                    < node_link.get_rule_limit_value().parse::<f64>().unwrap()
            }
            Rule::GE => {
                matter_value.parse::<f64>().unwrap()
                    >= node_link.get_rule_limit_value().parse::<f64>().unwrap()
            }
            Rule::LE => {
                matter_value.parse::<f64>().unwrap()
                    <= node_link.get_rule_limit_value().parse::<f64>().unwrap()
            }
            _ => false,
        }
    }

    fn matter_value(
        &self,
        tree_id: i64,
        user_id: &String,
        decision_matter: &HashMap<String, String>,
    ) -> String;
}
