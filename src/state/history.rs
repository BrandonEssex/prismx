use super::core::{AppState, LayoutSnapshot};

const UNDO_LIMIT: usize = 50;

impl AppState {
    pub fn push_undo(&mut self) {
        let snap = LayoutSnapshot {
            nodes: self.nodes.clone(),
            root_nodes: self.root_nodes.clone(),
            selected: self.selected,
        };
        if self.undo_stack.last().map(|s| s == &snap).unwrap_or(false) {
            return;
        }
        self.undo_stack.push(snap);
        if self.undo_stack.len() > UNDO_LIMIT {
            let excess = self.undo_stack.len() - UNDO_LIMIT;
            self.undo_stack.drain(0..excess);
        }
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            crate::log_debug!(self, "UNDO triggered: stack now = {}", self.undo_stack.len());
            let current = LayoutSnapshot {
                nodes: self.nodes.clone(),
                root_nodes: self.root_nodes.clone(),
                selected: self.selected,
            };
            self.redo_stack.push(current);
            self.nodes = prev.nodes;
            self.root_nodes = prev.root_nodes;
            self.selected = prev.selected;
            crate::layout::roles::recalculate_roles(self);
            self.ensure_valid_roots();
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            crate::log_debug!(self, "REDO triggered: stack now = {}", self.redo_stack.len());
            let current = LayoutSnapshot {
                nodes: self.nodes.clone(),
                root_nodes: self.root_nodes.clone(),
                selected: self.selected,
            };
            self.undo_stack.push(current);
            self.nodes = next.nodes;
            self.root_nodes = next.root_nodes;
            self.selected = next.selected;
            crate::layout::roles::recalculate_roles(self);
            self.ensure_valid_roots();
        }
    }
}
