#[derive(Debug, Clone)]
pub enum NodeAction {
    Create(u64),
    Delete(u64),
}

#[derive(Default)]
pub struct ActionStack {
    undo_stack: Vec<NodeAction>,
    redo_stack: Vec<NodeAction>,
}