// FINAL FULL FILE DELIVERY
// Filename: /src/mindmap_state.rs

use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub label: String,
    pub parent: Option<Uuid>,
    pub children: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MindmapState {
    pub nodes: HashMap<Uuid, Node>,
    pub root_id: Option<Uuid>,
    pub selected: Option<Uuid>,
    pub editing: Option<Uuid>,
    pub edit_buffer: String,
}

impl MindmapState {
    pub fn new() -> Self {
        let mut nodes = HashMap::new();
        let root_id = Uuid::new_v4();

        nodes.insert(root_id, Node {
            id: root_id,
            label: "Root".to_string(),
            parent: None,
            children: vec![],
        });

        Self {
            nodes,
            root_id: Some(root_id),
            selected: Some(root_id),
            editing: None,
            edit_buffer: String::new(),
        }
    }

    pub fn handle_action(&mut self, _action: crate::actions::Action) {
        // Action routing stub (no-op)
    }
}
