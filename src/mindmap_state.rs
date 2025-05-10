// FINAL FULL FILE DELIVERY
// Filename: /src/mindmap_state.rs
// File Delivery Progress: 7/∞ FINAL FILES delivered

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodeType {
    Note,
    Task,
    Idea,
    Custom(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub author: String,
    pub timestamp: String,
    pub content: String,
    pub replies: Vec<Comment>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub label: String,
    pub children: Vec<Node>,
    pub parent: Option<Uuid>,
    pub expanded: bool,
    pub node_type: NodeType,
    pub tags: Vec<String>,
    pub shard: Option<String>,
    pub references: Vec<String>,
    pub linked_ids: Vec<Uuid>,
    pub locked: bool,
    pub encrypted_payload: Option<String>,
    pub comments: Vec<Comment>,
}

impl Node {
    pub fn new(label: &str, parent: Option<Uuid>) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            label: label.to_string(),
            children: Vec::new(),
            parent,
            expanded: true,
            node_type: NodeType::Note,
            tags: vec![],
            shard: None,
            references: vec![],
            linked_ids: vec![],
            locked: false,
            encrypted_payload: None,
            comments: vec![],
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
        let root = Node::new("Root", None);
        let focused_node_id = root.id;
        Self { root, focused_node_id }
    }
}