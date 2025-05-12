// src/undo_redo.rs

use crate::node::Node;
use std::collections::VecDeque;

#[derive(Debug)]
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
        self.undo_stack.push_back(action);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> Option<NodeAction> {
        self.undo_stack.pop_back().map(|action| {
            self.redo_stack.push_back(action.clone());
            action
        })
    }

    pub fn redo(&mut self) -> Option<NodeAction> {
        self.redo_stack.pop_back().map(|action| {
            self.undo_stack.push_back(action.clone());
            action
        })
    }
}