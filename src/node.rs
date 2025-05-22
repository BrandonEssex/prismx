use std::collections::HashMap;

pub type NodeID = u64;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: NodeID,
    pub label: String,
    pub parent: Option<NodeID>,
    pub children: Vec<NodeID>,
    pub collapsed: bool,
    /// X coordinate when manually positioned
    pub x: i32,
    /// Y coordinate when manually positioned
    pub y: i32,
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
        }
    }
}

pub type NodeMap = HashMap<NodeID, Node>;
pub type Selection = Option<NodeID>;
