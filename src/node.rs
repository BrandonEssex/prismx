use std::collections::HashMap;

pub type NodeID = u64;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: NodeID,
    pub label: String,
    pub parent: Option<NodeID>,
    pub children: Vec<NodeID>,
    pub collapsed: bool,
}

impl Node {
    pub fn new(id: NodeID, label: &str, parent: Option<NodeID>) -> Self {
        Self {
            id,
            label: label.to_string(),
            parent,
            children: vec![],
            collapsed: false,
        }
    }
}

pub type NodeMap = HashMap<NodeID, Node>;
pub type Selection = Option<NodeID>;
