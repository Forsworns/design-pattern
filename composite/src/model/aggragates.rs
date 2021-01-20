use crate::{HashMap, TreeNode, TreeRoot};
#[derive(Debug)]
pub struct DecisionTree {
    root: TreeRoot,
    nodes: HashMap<i64, TreeNode>,
}

impl DecisionTree {
    pub fn new(root: TreeRoot, nodes: HashMap<i64, TreeNode>) -> Self {
        Self { root, nodes }
    }

    pub fn get_tree_root(&self) -> &TreeRoot {
        &self.root
    }

    pub fn get_tree_node_map(&self) -> &HashMap<i64, TreeNode> {
        &self.nodes
    }
}
