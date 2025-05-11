#[derive(Debug)]
pub enum NodeAction {
    AddNode,
    DeleteNode,
    EditNode(String),
}

#[derive(Debug)]
pub struct ActionStack {
    pub history: Vec<NodeAction>,
}

impl ActionStack {
    pub fn new() -> Self {
        ActionStack {
            history: Vec::new(),
        }
    }

    pub fn push(&mut self, action: NodeAction) {
        self.history.push(action);
    }
}