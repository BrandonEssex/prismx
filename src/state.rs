#[derive(Debug, Clone)]
pub struct Node {
    pub label: String,
    pub children: Vec<Node>,
}

pub struct AppState {
    pub mode: String,
    pub zen_buffer: Vec<String>,
    pub root: Node,
    pub flat_nodes: Vec<(usize, *mut Node)>, // (depth, pointer to Node)
    pub active_node: usize,
    pub edit_mode: bool,
    pub spotlight_input: String,
    pub show_spotlight: bool,
    pub show_triage: bool,
    pub show_keymap: bool,
}

impl Default for AppState {
    fn default() -> Self {
        let mut root = Node {
            label: "Root".into(),
            children: vec![
                Node {
                    label: "Node A".into(),
                    children: vec![],
                },
                Node {
                    label: "Node B".into(),
                    children: vec![],
                },
            ],
        };

        let flat = flatten_nodes(&mut root);

        Self {
            mode: "mindmap".into(),
            zen_buffer: vec!["".into()],
            root,
            flat_nodes: flat,
            active_node: 0,
            edit_mode: false,
            spotlight_input: String::new(),
            show_spotlight: false,
            show_triage: false,
            show_keymap: false,
        }
    }
}

pub fn flatten_nodes(root: &mut Node) -> Vec<(usize, *mut Node)> {
    fn recurse<'a>(node: *mut Node, depth: usize, out: &mut Vec<(usize, *mut Node)>) {
        unsafe {
            out.push((depth, node));
            for child in &mut (*node).children {
                recurse(child as *mut Node, depth + 1, out);
            }
        }
    }

    let mut flat = Vec::new();
    recurse(root as *mut Node, 0, &mut flat);
    flat
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
        if self.active_node + 1 < self.flat_nodes.len() {
            self.active_node += 1;
        }
    }

    pub fn get_active_node_mut(&mut self) -> Option<&mut Node> {
        self.flat_nodes
            .get(self.active_node)
            .map(|(_, ptr)| unsafe { &mut **ptr })
    }

    pub fn add_sibling_node(&mut self) {
        if let Some((_, ptr)) = self.flat_nodes.get(self.active_node) {
            unsafe {
                for parent in self.iter_mut_all_nodes() {
                    for i in 0..parent.children.len() {
                        let child_ptr = &mut parent.children[i] as *mut Node;
                        if child_ptr == *ptr {
                            parent
                                .children
                                .insert(i + 1, Node { label: "New Sibling".into(), children: vec![] });
                            self.reflatten();
                            return;
                        }
                    }
                }
            }
        }
    }

    pub fn add_child_node(&mut self) {
        if let Some(node) = self.get_active_node_mut() {
            node.children.push(Node {
                label: "New Child".into(),
                children: vec![],
            });
            self.reflatten();
        }
    }

    pub fn delete_node(&mut self) {
        if self.active_node == 0 {
            return; // Don't delete root
        }

        let target_ptr = self.flat_nodes[self.active_node].1;
        unsafe {
            for parent in self.iter_mut_all_nodes() {
                parent.children.retain(|c| c as *const Node != target_ptr);
            }
        }
        self.active_node = self.active_node.saturating_sub(1);
        self.reflatten();
    }

    pub fn reflatten(&mut self) {
        self.flat_nodes = flatten_nodes(&mut self.root);
    }

    pub fn iter_mut_all_nodes(&mut self) -> Vec<&mut Node> {
        fn recurse<'a>(node: &'a mut Node, out: &mut Vec<&'a mut Node>) {
            out.push(node);
            for child in &mut node.children {
                recurse(child, out);
            }
        }
        let mut result = vec![];
        recurse(&mut self.root, &mut result);
        result
    }
}
