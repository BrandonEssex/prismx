use crate::util::undo_redo::ActionStack;
use crate::util::undo_redo::NodeAction;

use ratatui::{
    text::Line,
    widgets::canvas::{Context},
};

#[derive(Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Node {
    pub id: u64,
    pub title: String,
    pub position: Position,
}

#[derive(Debug)]
pub struct Mindmap {
    pub nodes: std::collections::HashMap<u64, Node>,
}

#[derive(Debug)]
pub struct MindmapState {
    pub mindmap: Mindmap,
    pub undo_redo: ActionStack,
}

pub fn draw_mindmap_view<'a>(ctx: &mut Context<'a>, state: &'a MindmapState) {
    for node in state.mindmap.nodes.values() {
        ctx.print(node.position.x, node.position.y, Line::from(node.title.clone()));
    }
}