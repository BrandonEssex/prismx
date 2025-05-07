use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: u64,
    pub content: String,
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Mindmap {
    pub nodes: HashMap<u64, Node>,
    pub next_id: u64,
}

impl Mindmap {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_node(&mut self, content: &str, position: Position) -> u64 {
        let id = self.next_id;
        self.nodes.insert(id, Node {
            id,
            content: content.to_string(),
            position,
        });
        self.next_id += 1;
        id
    }

    pub fn remove_node(&mut self, id: u64) -> bool {
        self.nodes.remove(&id).is_some()
    }

    pub fn move_node(&mut self, id: u64, new_position: Position) -> bool {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.position = new_position;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
pub struct MindmapState {
    pub mindmap: Mindmap,
}