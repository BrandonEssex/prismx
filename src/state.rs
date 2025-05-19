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
            mode: "mindmap".into(),
            zen_buffer: vec!["".into()],
            mindmap_nodes: vec!["Root".into(), "Node A".into(), "Node B".into()],
            spotlight_input: String::new(),
            show_spotlight: false,
            show_triage: false,
            show_keymap: false,
        }
    }
}

impl AppState {
    pub fn execute_spotlight_command(&mut self) {
        let cmd = self.spotlight_input.trim();
        match cmd {
            "/toggle triage" => self.show_triage = !self.show_triage,
            "/toggle keymap" => self.show_keymap = !self.show_keymap,
            "/toggle spotlight" => self.show_spotlight = !self.show_spotlight,
            "/mode zen" => self.mode = "zen".into(),
            "/mode mindmap" => self.mode = "mindmap".into(),
            "/clear" => self.zen_buffer = vec![String::new()],
            _ => {}
        }
        self.spotlight_input.clear();
        self.show_spotlight = false;
    }
}
