use composite::{
    DecisionTree, EngineConfig, Rule, TreeEngineHandler, TreeNode, TreeNodeLink, TreeRoot,
};
use std::collections::HashMap;
#[test]
fn decision_test() {
    // use `cargo test -- --nocapture`
    let decision_tree = init();
    println!("{:?}\n", decision_tree);
    let tree_handler = TreeEngineHandler {};
    let mut decision_matter = HashMap::<String, String>::new();
    decision_matter.insert("gender".to_owned(), "man".to_owned());
    decision_matter.insert("age".to_owned(), "29".to_owned());
    let decision_result = tree_handler.process(
        10001,
        String::from("Oli09pLkdjh"),
        decision_tree,
        decision_matter,
    );
    println!("{:?}\n", decision_result);
}

fn init() -> DecisionTree {
    // node1 for gender decision
    let mut tree_node_01 = TreeNode::new(
        10001,
        1,
        true,
        None,
        "gender".to_owned(),
        "gender[man/woman]".to_owned(),
    );
    // link: 1->11, male
    let tree_node_link_11 = TreeNodeLink::new(1, 11, Rule::Eq, "man".to_owned());
    // link: 1->12, female
    let tree_node_link_12 = TreeNodeLink::new(1, 12, Rule::Eq, "woman".to_owned());
    // link list
    let tree_node_list_01 = vec![tree_node_link_11, tree_node_link_12];
    tree_node_01.set_tree_node_link_list(tree_node_list_01);

    // node11 for male age decision
    let mut tree_node_11 = TreeNode::new(
        10001,
        11,
        true,
        None,
        "age".to_owned(),
        "age[number".to_owned(),
    );
    // link: 11->111
    let tree_node_link_111 = TreeNodeLink::new(11, 111, Rule::G, "25".to_owned());
    // link: 11->112
    let tree_node_link_112 = TreeNodeLink::new(11, 112, Rule::LE, "25".to_owned());
    let tree_node_list_11 = vec![tree_node_link_111, tree_node_link_112];
    tree_node_11.set_tree_node_link_list(tree_node_list_11);

    // node12 for female age decision
    let mut tree_node_12 = TreeNode::new(
        10001,
        12,
        true,
        None,
        "age".to_owned(),
        "age[number]".to_owned(),
    );
    // link: 12->121
    let tree_node_link_121 = TreeNodeLink::new(12, 121, Rule::G, "25".to_owned());
    // link: 12->122
    let tree_node_link_122 = TreeNodeLink::new(12, 122, Rule::LE, "25".to_owned());
    let tree_node_link_list_12 = vec![tree_node_link_121, tree_node_link_122];
    tree_node_12.set_tree_node_link_list(tree_node_link_list_12);
    // node 111, decision result
    let tree_node_111 = TreeNode::new(
        10001,
        111,
        false,
        Some("leaf A".to_owned()),
        String::new(),
        String::new(),
    );
    // node 112, decision result
    let tree_node_112 = TreeNode::new(
        10001,
        112,
        false,
        Some("leaf B".to_owned()),
        String::new(),
        String::new(),
    );
    // node 121, decision result
    let tree_node_121 = TreeNode::new(
        10001,
        121,
        false,
        Some("leaf C".to_owned()),
        String::new(),
        String::new(),
    );
    // node 122, decision result
    let tree_node_122 = TreeNode::new(
        10001,
        122,
        false,
        Some("leaf D".to_owned()),
        String::new(),
        String::new(),
    );

    // root
    let tree_root = TreeRoot::new(10001, 1, "decision tree".to_owned());
    // node maps
    let mut tree_node_map = HashMap::<i64, TreeNode>::new();
    tree_node_map.insert(1, tree_node_01);
    tree_node_map.insert(11, tree_node_11);
    tree_node_map.insert(12, tree_node_12);
    tree_node_map.insert(111, tree_node_111);
    tree_node_map.insert(112, tree_node_112);
    tree_node_map.insert(121, tree_node_121);
    tree_node_map.insert(122, tree_node_122);
    DecisionTree::new(tree_root, tree_node_map)
}
