use crate::{DecisionTree, EngineBase, EngineConfig, EngineResult, HashMap};

pub struct TreeEngineHandler {}

impl EngineConfig for TreeEngineHandler {
    fn process(
        &self,
        tree_id: i64,
        user_id: String,
        tree: DecisionTree,
        decision_matter: HashMap<String, String>,
    ) -> EngineResult {
        let decision_node = self.engine_decision_maker(&tree, tree_id, &user_id, &decision_matter);
        EngineResult::new(
            user_id,
            tree_id,
            decision_node.get_node_id(),
            decision_node.get_node_value().unwrap(),
        )
    }
}

impl EngineBase for TreeEngineHandler {}
