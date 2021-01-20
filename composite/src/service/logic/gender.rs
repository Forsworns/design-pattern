use crate::{HashMap, LogicFilter};
#[derive(Debug)]
pub struct UserGenderFilter {}

impl LogicFilter for UserGenderFilter {
    fn matter_value(
        &self,
        _tree_id: i64,
        _user_id: &String,
        decision_matter: &HashMap<String, String>,
    ) -> String {
        decision_matter.get("gender").unwrap().clone()
    }
}
