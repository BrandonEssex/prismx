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

#[derive(Debug, Default)]
pub struct MindmapState {
    pub mindmap: Mindmap,
}