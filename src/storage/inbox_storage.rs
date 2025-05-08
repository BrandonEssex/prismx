#[derive(Debug, Clone, Default)]
pub struct InboxState {
    pub tasks: Vec<String>,
}

impl InboxState {
    pub fn new() -> Self {
        InboxState {
            tasks: vec!["Review logs".to_string()],
        }
    }
}
