#[derive(Debug, Clone)]
pub struct Node {
    pub label: String,
}

pub struct Mindmap {
    pub nodes: Vec<Node>,
    pub active_index: usize,
}

impl Mindmap {
    pub fn new() -> Self {
        Self {
            nodes: vec![
                Node { label: "Root".into() },
                Node { label: "Idea A".into() },
                Node { label: "Idea B".into() },
            ],
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
