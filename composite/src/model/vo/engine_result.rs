#[derive(Debug)]
pub struct EngineResult {
    user_id: String,
    tree_id: i64,
    node_id: i64,
    node_value: String,
}

impl EngineResult {
    pub fn new(user_id: String, tree_id: i64, node_id: i64, node_value: String) -> Self {
        Self {
            user_id,
            tree_id,
            node_id,
            node_value,
        }
    }
}
