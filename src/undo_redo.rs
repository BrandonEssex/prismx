// src/undo_redo.rs

use crate::node::Node;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum NodeAction {
    Add(Node),
    Remove(Node),
    Edit(Node),
}

#[derive(Debug, Default)]
pub struct ActionStack {
    pub undo_stack: VecDeque<NodeAction>,
    pub redo_stack: VecDeque<NodeAction>,
}

impl ActionStack {
    pub fn push_undo(&mut self, action: NodeAction) {
        self.undo_stack.push_back(action.clone());
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> Option<NodeAction> {
        if let Some(action) = self.undo_stack.pop_back() {
            self.redo_stack.push_back(action.clone());
            Some(action)
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<NodeAction> {
        if let Some(action) = self.redo_stack.pop_back() {
            self.undo_stack.push_back(action.clone());
            Some(action)
        } else {
            None
        }
    }
}