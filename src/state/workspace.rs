use crate::node::{NodeMap, NodeID};
use crate::storage::json_io::{WorkspaceData, save_workspace, load_workspace};

pub struct WorkspaceSet {
    pub spaces: Vec<WorkspaceData>,
    pub current: usize,
}

impl WorkspaceSet {
    pub fn new(initial: WorkspaceData) -> Self {
        Self { spaces: vec![initial], current: 0 }
    }

    pub fn current_data(&self) -> WorkspaceData {
        self.spaces.get(self.current).cloned().unwrap_or_default()
    }

    pub fn replace_current(&mut self, data: WorkspaceData) {
        if self.current >= self.spaces.len() {
            self.spaces.push(data);
        } else {
            self.spaces[self.current] = data;
        }
    }

    pub fn switch_next(&mut self) {
        if self.spaces.is_empty() {
            return;
        }
        self.current = (self.current + 1) % self.spaces.len();
    }

    pub fn save_current(&self) {
        let _ = save_workspace(&self.spaces[self.current]);
    }

    pub fn load_into_current(&mut self) {
        if let Ok(data) = load_workspace() {
            if self.current < self.spaces.len() {
                self.spaces[self.current] = data;
            } else {
                self.spaces.push(data);
                self.current = self.spaces.len() - 1;
            }
        }
    }
}
