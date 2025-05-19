pub struct Mindmap {
    pub nodes: Vec<String>,
    pub active_index: usize,
}

impl Mindmap {
    pub fn new() -> Self {
        Self {
            nodes: vec!["Root".into(), "Idea A".into(), "Idea B".into()],
            active_index: 0,
        }
    }

    pub fn move_next(&mut self) {
        if self.active_index + 1 < self.nodes.len() {
            self.active_index += 1;
        }
    }

    pub fn move_prev(&mut self) {
        if self.active_index > 0 {
            self.active_index -= 1;
        }
    }
}
