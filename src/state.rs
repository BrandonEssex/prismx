pub struct AppState {
    pub mode: String,
    pub zen_buffer: Vec<String>,
    pub mindmap_nodes: Vec<String>,
    pub active_node: usize,
    pub edit_mode: bool,
    pub edit_buffer: String,
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
            active_node: 0,
            edit_mode: false,
            edit_buffer: String::new(),
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

    pub fn move_focus_up(&mut self) {
        if self.active_node > 0 {
            self.active_node -= 1;
        }
    }

    pub fn move_focus_down(&mut self) {
        if self.active_node + 1 < self.mindmap_nodes.len() {
            self.active_node += 1;
        }
    }

    pub fn add_sibling_node(&mut self) {
        let label = String::from("New Sibling");
        let insert_at = self.active_node + 1;
        if insert_at <= self.mindmap_nodes.len() {
            self.mindmap_nodes.insert(insert_at, label);
        }
    }

    pub fn add_child_node(&mut self) {
        let label = String::from("New Child");
        self.mindmap_nodes.insert(self.active_node + 1, label);
    }

    pub fn delete_node(&mut self) {
        if self.active_node > 0 && self.active_node < self.mindmap_nodes.len() {
            self.mindmap_nodes.remove(self.active_node);
            self.active_node = self.active_node.saturating_sub(1);
        }
    }
}
