use crate::util::undo_redo::{ActionStack, NodeAction};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, canvas::{Canvas, Context}},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: u64,
    pub title: String,
    pub position: Position,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Mindmap {
    pub nodes: HashMap<u64, Node>,
    pub next_id: u64,
}

impl Mindmap {
    pub fn add_node(&mut self, title: &str, position: Position) -> u64 {
        let id = self.next_id;
        self.nodes.insert(id, Node {
            id,
            title: title.to_string(),
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

#[derive(Debug)]
pub struct MindmapState {
    pub mindmap: Mindmap,
    pub undo_redo: ActionStack,
}

impl MindmapState {
    pub fn new() -> Self {
        Self {
            mindmap: Mindmap::default(),
            undo_redo: ActionStack::new(),
        }
    }
}

pub fn draw_mindmap_view(ctx: &mut Context, state: &MindmapState) {
    for node in state.mindmap.nodes.values() {
        ctx.print(node.position.x, node.position.y, &*node.title);
    }
}