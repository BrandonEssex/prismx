use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub label: String,
    pub children: Vec<Node>,
    pub parent: Option<Uuid>,
    pub expanded: bool,
}

impl Node {
    pub fn new(label: &str, parent: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            label: label.to_string(),
            children: Vec::new(),
            parent,
            expanded: true,
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MindmapState {
    pub root: Node,
    pub focused_node_id: Uuid,
}

impl MindmapState {
    pub fn new() -> Self {
        let root = Node::new(\"Root\", None);
        let focused_node_id = root.id;
        Self { root, focused_node_id }
    }
}