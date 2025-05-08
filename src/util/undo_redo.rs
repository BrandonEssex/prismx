// FINAL VERSION — File Delivery Progress: 6/6  
// File: src/util/undo_redo.rs

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum NodeAction {
    CreateNode(u64),
    DeleteNode(u64),
    MoveNode(u64, f64, f64),
    UpdateContent(u64, String),
}

#[derive(Debug)]
pub struct ActionStack {
    undo_stack: VecDeque<NodeAction>,
    redo_stack: VecDeque<NodeAction>,
}

impl ActionStack {
    pub fn new() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn push(&mut self, action: NodeAction) {
        self.undo_stack.push_back(action);
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