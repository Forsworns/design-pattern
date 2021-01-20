pub type Id = i64;
#[derive(Clone, Debug)]
pub enum Rule {
    Eq,
    G,
    L,
    GE,
    LE,
    None,
}

#[derive(Clone, Debug)]
pub struct TreeNode {
    tree_id: Id,
    id: Id,
    node_type: bool,
    value: Option<String>,
    rule_key: String,
    rule_description: String,
    tree_node_link_list: Vec<TreeNodeLink>,
}

impl TreeNode {
    pub fn new(
        tree_id: Id,
        id: Id,
        node_type: bool,
        value: Option<String>,
        rule_key: String,
        rule_description: String,
    ) -> Self {
        Self {
            id,
            tree_id,
            node_type,
            value,
            rule_key,
            rule_description,
            tree_node_link_list: Vec::<TreeNodeLink>::new(),
        }
    }

    pub fn get_node_type(&self) -> bool {
        self.node_type
    }

    pub fn get_rule_key(&self) -> String {
        self.rule_key.clone()
    }

    pub fn get_tree_node_link_list(&self) -> &Vec<TreeNodeLink> {
        &self.tree_node_link_list
    }

    pub fn get_node_id(&self) -> Id {
        self.id
    }

    pub fn get_node_value(&self) -> Option<String> {
        self.value.clone()
    }

    pub fn set_tree_node_link_list(&mut self, list: Vec<TreeNodeLink>) {
        self.tree_node_link_list = list;
    }
}

#[derive(Clone, Debug)]
pub struct TreeNodeLink {
    node_id_from: Id,
    node_id_to: Id,
    rule_limit_type: Rule,
    rule_limit_value: String,
}

impl TreeNodeLink {
    pub fn new(
        node_id_from: Id,
        node_id_to: Id,
        rule_limit_type: Rule,
        rule_limit_value: String,
    ) -> Self {
        Self {
            node_id_from,
            node_id_to,
            rule_limit_type,
            rule_limit_value,
        }
    }

    pub fn get_node_id_to(&self) -> Id {
        self.node_id_to
    }

    pub fn get_rule_limit_type(&self) -> Rule {
        self.rule_limit_type.clone()
    }

    pub fn get_rule_limit_value(&self) -> String {
        self.rule_limit_value.to_owned()
    }
}

#[derive(Debug)]
pub struct TreeRoot {
    tree_id: Id,
    root_id: Id,
    name: String,
}

impl TreeRoot {
    pub fn new(tree_id: Id, root_id: Id, name: String) -> Self {
        Self {
            tree_id,
            root_id,
            name,
        }
    }

    pub fn get_id(&self) -> Id {
        self.root_id
    }
}
