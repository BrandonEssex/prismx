// FINAL FULL FILE DELIVERY
// Filename: /src/mindmap_state.rs

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::actions::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub label: String,
    pub parent: Option<Uuid>,
    pub children: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindmapState {
    pub nodes: HashMap<Uuid, Node>,
    pub root_id: Option<Uuid>,
    pub selected: Option<Uuid>,
    pub editing: Option<Uuid>,
    pub edit_buffer: String,
}

impl MindmapState {
    pub fn new() -> Self {
        let mut state = Self {
            nodes: HashMap::new(),
            root_id: None,
            selected: None,
            editing: None,
            edit_buffer: String::new(),
        };

        let root = Node {
            id: Uuid::new_v4(),
            label: "Root".to_string(),
            parent: None,
            children: vec![],
        };

        let root_id = root.id;
        state.nodes.insert(root_id, root);
        state.root_id = Some(root_id);
        state.selected = Some(root_id);
        state
    }

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::EnterEditNode => {
                if let Some(id) = self.selected {
                    if let Some(node) = self.nodes.get(&id) {
                        self.edit_buffer = node.label.clone();
                        self.editing = Some(id);
                    }
                }
            }
            Action::PushEditChar(c) => {
                self.edit_buffer.push(c);
            }
            Action::PopEditChar => {
                self.edit_buffer.pop();
            }
            Action::CommitEdit => {
                if let Some(id) = self.editing.take() {
                    if let Some(node) = self.nodes.get_mut(&id) {
                        node.label = self.edit_buffer.clone();
                        self.edit_buffer.clear();
                    }
                }
            }
            Action::CancelEdit => {
                self.editing = None;
                self.edit_buffer.clear();
            }
            Action::NavigateNext => {
                self.select_next();
            }
            Action::NavigatePrev => {
                self.select_prev();
            }
            _ => {}
        }
    }

    pub fn select_next(&mut self) {
        if let Some(current_id) = self.selected {
            let mut ids: Vec<_> = self.nodes.keys().cloned().collect();
            ids.sort();
            if let Some(pos) = ids.iter().position(|id| *id == current_id) {
                let next_index = (pos + 1) % ids.len();
                self.selected = Some(ids[next_index]);
            }
        }
    }

    pub fn select_prev(&mut self) {
        if let Some(current_id) = self.selected {
            let mut ids: Vec<_> = self.nodes.keys().cloned().collect();
            ids.sort();
            if let Some(pos) = ids.iter().position(|id| *id == current_id) {
                let prev_index = if pos == 0 { ids.len() - 1 } else { pos - 1 };
                self.selected = Some(ids[prev_index]);
            }
        }
    }
}