pub struct Node {
    pub label: String,
    pub children: Vec<Node>,
}

pub struct AppState {
    pub mode: String,
    pub zen_buffer: Vec<String>,
    pub root: Node,
    pub flat_nodes: Vec<(usize, String)>, // (depth, label)
    pub active_node: usize,
    pub edit_mode: bool,
    pub spotlight_input: String,
    pub show_spotlight: bool,
    pub show_triage: bool,
    pub show_keymap: bool,
}

impl Default for AppState {
    fn default() -> Self {
        let root = Node {
            label: "Root".into(),
            children: vec![
                Node { label: "Node A".into(), children: vec![] },
                Node { label: "Node B".into(), children: vec![] },
            ],
        };

        let flat = flatten_nodes(&root);

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

pub fn flatten_nodes(node: &Node) -> Vec<(usize, String)> {
    fn recurse(n: &Node, depth: usize, out: &mut Vec<(usize, String)>) {
        out.push((depth, n.label.clone()));
        for child in &n.children {
            recurse(child, depth + 1, out);
        }
    }

    let mut result = Vec::new();
    recurse(node, 0, &mut result);
    result
}
