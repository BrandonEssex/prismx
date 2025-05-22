use std::collections::HashMap;
use crate::node::{Node, NodeID, NodeMap};

mod hotkeys;
pub use hotkeys::*;

pub struct AppState {
    pub mode: String,
    pub zen_buffer: Vec<String>,
    pub nodes: NodeMap,
    pub root_nodes: Vec<NodeID>,
    pub selected: Option<NodeID>,
    pub spotlight_input: String,
    pub show_spotlight: bool,
    pub show_triage: bool,
    pub show_keymap: bool,
    pub module_switcher_open: bool,
    pub module_switcher_index: usize,
    pub hotkeys: HashMap<String, String>,
    pub scroll_offset: usize,
    pub max_visible_lines: usize,
}

impl Default for AppState {
    fn default() -> Self {
        let mut nodes = NodeMap::new();
        let node_a = 1;
        let node_b = 2;

        nodes.insert(node_a, Node::new(node_a, "Node A", None));
        nodes.insert(node_b, Node::new(node_b, "Node B", None));

        Self {
            mode: "gemx".into(),
            zen_buffer: vec![String::from(" ")],
            nodes,
            root_nodes: vec![node_a, node_b],
            selected: Some(node_a),
            spotlight_input: String::new(),
            show_spotlight: false,
            show_triage: false,
            show_keymap: false,
            module_switcher_open: false,
            module_switcher_index: 0,
            hotkeys: load_default_hotkeys(),
            scroll_offset: 0,
            max_visible_lines: 20,
        }
    }
}

impl AppState {
    pub fn get_selected_node(&self) -> Option<&Node> {
        self.selected.and_then(|id| self.nodes.get(&id))
    }

    pub fn get_selected_node_mut(&mut self) -> Option<&mut Node> {
        self.selected.and_then(|id| self.nodes.get_mut(&id))
    }

    pub fn visible_node_order(&self) -> Vec<NodeID> {
        let mut result = Vec::new();

        fn walk(id: NodeID, nodes: &NodeMap, out: &mut Vec<NodeID>) {
            out.push(id);
            if let Some(node) = nodes.get(&id) {
                if node.collapsed {
                    return;
                }
                for child in &node.children {
                    walk(*child, nodes, out);
                }
            }
        }

        for root_id in &self.root_nodes {
            if self.nodes.contains_key(root_id) {
                walk(*root_id, &self.nodes, &mut result);
            }
        }

        result
    }

    pub fn move_focus_up(&mut self) {
        if let Some(current) = self.selected {
            let order = self.visible_node_order();
            if let Some(pos) = order.iter().position(|&id| id == current) {
                if pos > 0 {
                    self.selected = Some(order[pos - 1]);
                }
            }
        }
    }

    pub fn move_focus_down(&mut self) {
        if let Some(current) = self.selected {
            let order = self.visible_node_order();
            if let Some(pos) = order.iter().position(|&id| id == current) {
                if pos + 1 < order.len() {
                    self.selected = Some(order[pos + 1]);
                }
            }
        }
    }

    pub fn move_focus_left(&mut self) {
        if let Some(current) = self.selected {
            if let Some(node) = self.nodes.get(&current) {
                if let Some(parent_id) = node.parent {
                    self.selected = Some(parent_id);
                }
            }
        }
    }

    pub fn move_focus_right(&mut self) {
        if let Some(current) = self.selected {
            if let Some(node) = self.nodes.get(&current) {
                if !node.children.is_empty() {
                    self.selected = Some(node.children[0]);
                }
            }
        }
    }

    pub fn update_active_label(&mut self, c: char) {
        if let Some(node) = self.get_selected_node_mut() {
            node.label.push(c);
        }
    }

    pub fn delete_last_char(&mut self) {
        if let Some(node) = self.get_selected_node_mut() {
            node.label.pop();
        }
    }

    pub fn add_child(&mut self) {
        if let Some(parent_id) = self.selected {
            let new_id = self.nodes.keys().max().copied().unwrap_or(100) + 1;
            let child = Node::new(new_id, "New Child", Some(parent_id));
            self.nodes.insert(new_id, child);
            if let Some(parent) = self.nodes.get_mut(&parent_id) {
                parent.children.push(new_id);
            }
            self.selected = Some(new_id);
        }
    }

    pub fn add_sibling(&mut self) {
        if let Some(selected_id) = self.selected {
            if let Some(selected_node) = self.nodes.get(&selected_id) {
                let parent_id = selected_node.parent;
                let new_id = self.nodes.keys().max().copied().unwrap_or(100) + 1;
                let sibling = Node::new(new_id, "New Sibling", parent_id);
                self.nodes.insert(new_id, sibling);

                match parent_id {
                    Some(p_id) => {
                        if let Some(parent) = self.nodes.get_mut(&p_id) {
                            parent.children.push(new_id);
                        }
                    }
                    None => {
                        self.root_nodes.push(new_id);
                    }
                }

                self.selected = Some(new_id);
            }
        }
    }

    pub fn delete_node(&mut self) {
        if let Some(target_id) = self.selected {
            let parent_id = self.nodes.get(&target_id).and_then(|n| n.parent);
            if let Some(pid) = parent_id {
                if let Some(parent) = self.nodes.get_mut(&pid) {
                    parent.children.retain(|&id| id != target_id);
                }
            } else {
                self.root_nodes.retain(|&id| id != target_id);
            }

            fn delete_recursive(map: &mut NodeMap, id: NodeID) {
                if let Some(node) = map.remove(&id) {
                    for child in node.children {
                        delete_recursive(map, child);
                    }
                }
            }

            delete_recursive(&mut self.nodes, target_id);
            self.selected = parent_id.or_else(|| self.root_nodes.first().copied());
        }
    }

    pub fn toggle_collapse(&mut self) {
        if let Some(node) = self.get_selected_node_mut() {
            node.collapsed = !node.collapsed;
        }
    }

    pub fn execute_spotlight_command(&mut self) {
        let cmd = self.spotlight_input.trim();
        match cmd {
            "/toggle triage" => self.show_triage = !self.show_triage,
            "/toggle keymap" => self.show_keymap = !self.show_keymap,
            "/toggle spotlight" => self.show_spotlight = !self.show_spotlight,
            "/mode zen" => self.mode = "zen".into(),
            "/mode gemx" => self.mode = "gemx".into(),
            "/clear" => self.zen_buffer = vec![String::new()],
            _ => {}
        }
        self.spotlight_input.clear();
        self.show_spotlight = false;
    }

    pub fn add_free_node(&mut self) {
        let new_id = self.nodes.keys().max().copied().unwrap_or(100) + 1;
        let node = Node::new(new_id, "Free Node", None);
        self.nodes.insert(new_id, node);
        self.root_nodes.push(new_id);
        self.selected = Some(new_id);
    }

    pub fn drill_down(&mut self) {
        if let Some(current) = self.get_selected_node() {
            if !current.children.is_empty() {
                self.selected = Some(current.children[0]);
            }
        }
    }

    pub fn export_zen_to_file(&self) {
        use std::fs::{self, File};
        use std::io::Write;
        use dirs;

        let path = dirs::document_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("prismx")
            .join("zen_export.md");

        let content = self.zen_buffer.join("\n");

        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(mut file) = File::create(&path) {
            let _ = file.write_all(content.as_bytes());
        }
    }

    pub fn get_module_by_index(&self) -> &str {
        match self.module_switcher_index % 4 {
            0 => "gemx",
            1 => "zen",
            2 => "settings",
            3 => "triage",
            _ => "gemx",
        }
    }
}
