pub struct AppState {
    pub mode: String,
    pub zen_buffer: Vec<String>,
    pub mindmap_nodes: Vec<String>,
    pub spotlight_input: String,
    pub show_spotlight: bool,
    pub show_triage: bool,
    pub show_keymap: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            mode: "zen".into(),
            zen_buffer: vec!["".into()],
            mindmap_nodes: vec!["Root".into(), "Child A".into(), "Child B".into()],
            spotlight_input: String::new(),
            show_spotlight: false,
            show_triage: false,
            show_keymap: false,
        }
    }
}
