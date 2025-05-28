use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub type NodeID = u64;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: NodeID,
    pub label: String,
    pub parent: Option<NodeID>,
    pub children: Vec<NodeID>,
    pub collapsed: bool,
    pub x: i16,
    pub y: i16,
}

impl Node {
    pub fn new(id: NodeID, label: &str, parent: Option<NodeID>) -> Self {
        Self {
            id,
            label: label.to_string(),
            parent,
            children: Vec::new(),
            collapsed: false,
            x: 0,
            y: 0,
        }
    }
}

pub type NodeMap = BTreeMap<NodeID, Node>;
pub type Selection = Option<NodeID>;
