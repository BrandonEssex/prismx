use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Node {
    pub label: String,
    pub children: Vec<Rc<RefCell<Node>>>,
}

pub struct AppState {
    pub mode: String,
    pub zen_buffer: Vec<String>,
    pub root: Rc<RefCell<Node>>,
    pub flat_nodes: Vec<(usize, Rc<RefCell<Node>>)>,
    pub active_node: usize,
    pub edit_mode: bool,
    pub spotlight_input: String,
    pub show_spotlight: bool,
    pub show_triage: bool,
    pub show_keymap: bool,
}

impl Default for AppState {
    fn default() -> Self {
        let root = Rc::new(RefCell::new(Node {
            label: "Root".into(),
            children: vec![
                Rc::new(RefCell::new(Node {
                    label: "Node A".into(),
                    children: vec![],
                })),
                Rc::new(RefCell::new(Node {
                    label: "Node B".into(),
                    children: vec![],
                })),
            ],
        }));

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

pub fn flatten_nodes(node: &Rc<RefCell<Node>>) -> Vec<(usize, Rc<RefCell<Node>>)> {
    fn recurse(
        node: &Rc<RefCell<Node>>,
        depth: usize,
        out: &mut Vec<(usize, Rc<RefCell<Node>>)>
    ) {
        out.push((depth, Rc::clone(node)));
        for child in &node.borrow().children {
            recurse(child, depth + 1, out);
        }
    }

    let mut result = Vec::new();
    recurse(node, 0, &mut result);
    result
}

impl AppState {
    pub fn reflatten(&mut self) {
        self.flat_nodes = flatten_nodes(&self.root);
    }

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

    pub fn get_active_node(&self) -> Rc<RefCell<Node>> {
        self.flat_nodes[self.active_node].1.clone()
    }

    pub fn update_active_label(&mut self, c: char) {
        let node = self.get_active_node();
        node.borrow_mut().label.push(c);
    }

    pub fn delete_last_char(&mut self) {
        let node = self.get_active_node();
        node.borrow_mut().label.pop();
    }

    pub fn add_sibling(&mut self) {
        if self.active_node == 0 {
            return;
        }

        let (target_depth, _) = self.flat_nodes[self.active_node];
        let mut parent_to_update: Option<Rc<RefCell<Node>>> = None;

        for (depth, node) in self.flat_nodes.iter().rev() {
            if *depth == target_depth - 1 {
                parent_to_update = Some(Rc::clone(node));
                break;
            }
        }

        if let Some(parent) = parent_to_update {
            parent.borrow_mut().children.push(Rc::new(RefCell::new(Node {
                label: "New Sibling".into(),
                children: vec![],
            })));
            self.reflatten();
        }
    }

    pub fn add_child(&mut self) {
        let node = self.get_active_node();
        node.borrow_mut().children.push(Rc::new(RefCell::new(Node {
            label: "New Child".into(),
            children: vec![],
        })));
        self.reflatten();
    }

    pub fn delete_node(&mut self) {
        if self.active_node == 0 {
            return; // can't delete root
        }

        let target = self.flat_nodes[self.active_node].1.clone();

        fn recurse_delete(node: &Rc<RefCell<Node>>, target: &Rc<RefCell<Node>>) -> bool {
            let mut n = node.borrow_mut();
            let len = n.children.len();
            n.children.retain(|child| !Rc::ptr_eq(child, target));
            if n.children.len() < len {
                return true;
            }
            for child in &n.children {
                if recurse_delete(child, target) {
                    return true;
                }
            }
            false
        }

        recurse_delete(&self.root, &target);
        self.active_node = self.active_node.saturating_sub(1);
        self.reflatten();
    }
}
