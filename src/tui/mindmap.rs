use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: u64,
    pub content: String,
    pub position: Position,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Mindmap {
    pub nodes: Vec<Node>,
    pub next_id: u64,
}

impl Mindmap {
    pub fn add_node(&mut self, content: &str, position: Position) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.push(Node {
            id,
            content: content.to_string(),
            position,
        });
        id
    }

    pub fn remove_node(&mut self, id: u64) -> bool {
        if let Some(pos) = self.nodes.iter().position(|n| n.id == id) {
            self.nodes.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn move_node(&mut self, id: u64, new_position: Position) -> bool {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == id) {
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

impl MindmapState {
    pub fn new() -> Self {
        Self {
            mindmap: Mindmap::default(),
        }
    }
}