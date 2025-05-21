use std::collections::HashMap;
use crate::node::{Node, NodeID, NodeMap};

mod hotkeys;

pub use hotkeys::*;

pub struct AppState {
    pub mode: String,
    pub zen_buffer: Vec<String>,
    pub nodes: NodeMap,
    pub root_id: NodeID,
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
        let root_id = 1;
        let child_a = 2;
        let child_b = 3;

        nodes.insert(root_id, Node::new(root_id, "Gemx Root", None));
        nodes.insert(child_a, Node::new(child_a, "Node A", Some(root_id)));
        nodes.insert(child_b, Node::new(child_b, "Node B", Some(root_id)));

        nodes.get_mut(&root_id).unwrap().children = vec![child_a, child_b];

        Self {
            mode: "gemx".into(),
            zen_buffer: vec![String::from(" ")],
            nodes,
            root_id,
            selected: Some(child_a),
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

    pub fn move_focus_up(&mut self) {
        // Placeholder: in Patch 25.30 this will become graph-aware
    }

    pub fn move_focus_down(&mut self) {
        // Placeholder: in Patch 25.30 this will become graph-aware
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
                if let Some(parent_id) = selected_node.parent {
                    let new_id = self.nodes.keys().max().copied().unwrap_or(100) + 1;
                    let sibling = Node::new(new_id, "New Sibling", Some(parent_id));
                    self.nodes.insert(new_id, sibling);
                    if let Some(parent) = self.nodes.get_mut(&parent_id) {
                        parent.children.push(new_id);
                    }
                    self.selected = Some(new_id);
                }
            }
        }
    }

    pub fn delete_node(&mut self) {
        if let Some(target_id) = self.selected {
            if target_id == self.root_id {
                return; // never delete root
            }

            let parent_id = self.nodes.get(&target_id).and_then(|n| n.parent);
            if let Some(parent_id) = parent_id {
                if let Some(parent) = self.nodes.get_mut(&parent_id) {
                    parent.children.retain(|&id| id != target_id);
                }
            }

            // Recursive delete of target and its children
            fn delete_recursive(map: &mut NodeMap, id: NodeID) {
                if let Some(node) = map.remove(&id) {
                    for child in node.children {
                        delete_recursive(map, child);
                    }
                }
            }

            delete_recursive(&mut self.nodes, target_id);
            self.selected = parent_id;
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
        let node = Node::new(new_id, "Free Node", Some(self.root_id));
        self.nodes.insert(new_id, node);
        if let Some(root) = self.nodes.get_mut(&self.root_id) {
            root.children.push(new_id);
        }
        self.selected = Some(new_id);
    }

    pub fn drill_down(&mut self) {
        if let Some(current) = self.get_selected_node() {
            if !current.children.is_empty() {
                self.selected = Some(current.children[0]);
            }
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
