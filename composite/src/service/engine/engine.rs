use crate::{DecisionTree, EngineResult, HashMap, TreeNode, LOGIC_FILTER_MAP};

pub trait EngineConfig {
    fn process(
        &self,
        tree_id: i64,
        user_id: String,
        tree: DecisionTree,
        decision_matter: HashMap<String, String>,
    ) -> EngineResult;
}

pub trait EngineBase: EngineConfig + Sized {
    fn engine_decision_maker(
        &self,
        tree: &DecisionTree,
        tree_id: i64,
        user_id: &String,
        decision_matter: &HashMap<String, String>,
    ) -> TreeNode {
        let tree_root = tree.get_tree_root();
        let tree_node_map = tree.get_tree_node_map();
        let root_node_id = tree_root.get_id();
        let mut tree_node_info = tree_node_map.get(&root_node_id).unwrap();
        while tree_node_info.get_node_type() {
            let rule_key = tree_node_info.get_rule_key();
            let logic_filters = LOGIC_FILTER_MAP.lock().unwrap();
            let logic_filter = logic_filters.get(&rule_key).unwrap();
            let matter_value = logic_filter.matter_value(tree_id, &user_id, &decision_matter);
            let next_node =
                logic_filter.filter(&matter_value, &tree_node_info.get_tree_node_link_list());
            tree_node_info = tree_node_map.get(&next_node).unwrap();
        }
        tree_node_info.clone()
    }
}
