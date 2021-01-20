use crate::{HashMap, LogicFilter};
#[derive(Debug)]
pub struct UserAgeFilter {}

impl LogicFilter for UserAgeFilter {
    fn matter_value(
        &self,
        _tree_id: i64,
        _user_id: &String,
        decision_matter: &HashMap<String, String>,
    ) -> String {
        decision_matter.get("age").unwrap().clone()
    }
}
