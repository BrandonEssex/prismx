use std::collections::HashMap;

pub type NodeID = u64;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: NodeID,
    pub label: String,
    pub parent: Option<NodeID>,
    pub children: Vec<NodeID>,
    pub collapsed: bool,
    pub x: i16,
    pub y: i16,
    pub is_positioned: bool,
}

impl Node {
    pub fn new(id: NodeID, label: &str, parent: Option<NodeID>) -> Self {
        Self {
            id,
            label: label.to_string(),
            parent,
            children: vec![],
            collapsed: false,
            x: 0,
            y: 0,
            is_positioned: false,
        }
    }
}

pub type NodeMap = HashMap<NodeID, Node>;
pub type Selection = Option<NodeID>;
