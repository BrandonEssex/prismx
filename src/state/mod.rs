use std::collections::{HashMap, HashSet, VecDeque};
use crate::node::{Node, NodeID, NodeMap};
use crate::layout::{ SIBLING_SPACING_X, CHILD_SPACING_Y, GEMX_HEADER_HEIGHT, LayoutRole };
use crossterm::terminal;
use crate::plugin::PluginHost;

mod hotkeys;
pub use hotkeys::*;



#[derive(Clone, Default)]
pub struct FavoriteEntry {
    pub icon: &'static str,
    pub command: &'static str,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DockLayout {
    Vertical,
    Horizontal,
}

pub struct AppState {
    pub mode: String,
    pub zen_buffer: Vec<String>,
    pub nodes: NodeMap,
    pub root_nodes: Vec<NodeID>,
    pub last_promoted_root: Option<NodeID>,
    pub selected: Option<NodeID>,
    pub spotlight_input: String,
    pub show_spotlight: bool,
    pub prev_show_spotlight: bool,
    pub spotlight_just_opened: bool,
    pub spotlight_animation_frame: u8,
    pub spotlight_history: VecDeque<String>,
    pub spotlight_history_index: Option<usize>,
    pub show_triage: bool,
    pub show_keymap: bool,
    pub module_switcher_open: bool,
    pub module_switcher_index: usize,
    pub hotkeys: HashMap<String, String>,
    pub scroll_offset: usize,
    pub max_visible_lines: usize,
    pub undo_stack: Vec<NodeMap>,
    pub redo_stack: Vec<NodeMap>,
    pub selected_drag_source: Option<NodeID>,
    pub link_map: std::collections::HashMap<NodeID, Vec<NodeID>>,
    pub auto_arrange: bool,
    pub zoom_scale: f32,
    pub zoom_locked_by_user: bool,
    pub scroll_x: i16,
    pub scroll_y: i16,
    pub snap_to_grid: bool,
    pub drawing_root: Option<NodeID>,
    pub dragging: Option<NodeID>,
    pub last_mouse: Option<(i16, i16)>,
    pub fallback_this_frame: bool,
    pub fallback_promoted_this_session: HashSet<NodeID>,
    pub layout_roles: HashMap<NodeID, LayoutRole>,
    pub layout_warning_logged: bool,
    pub debug_input_mode: bool,
    pub debug_border: bool,
    pub status_message: String,
    pub status_message_last_updated: Option<std::time::Instant>,
    pub plugin_host: PluginHost,
    pub plugin_favorites: Vec<FavoriteEntry>,
    pub favorite_dock_limit: usize,
    pub favorite_dock_layout: DockLayout,
    pub favorite_dock_enabled: bool,
    pub last_mouse_click: Option<(u16, u16)>,
    pub settings_focus_index: usize,

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
            last_promoted_root: None,
            selected: Some(node_a),
            spotlight_input: String::new(),
            show_spotlight: false,
            prev_show_spotlight: false,
            spotlight_just_opened: false,
            spotlight_animation_frame: 0,
            spotlight_history: VecDeque::new(),
            spotlight_history_index: None,
            show_triage: false,
            show_keymap: false,
            module_switcher_open: false,
            module_switcher_index: 0,
            hotkeys: load_default_hotkeys(),
            scroll_offset: 0,
            max_visible_lines: 20,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            selected_drag_source: None,
            link_map: std::collections::HashMap::new(),
            auto_arrange: true,
            zoom_scale: 1.0,
            zoom_locked_by_user: false,
            scroll_x: 0,
            scroll_y: 0,
            snap_to_grid: false,
            drawing_root: None,
            dragging: None,
            last_mouse: None,
            fallback_this_frame: false,
            fallback_promoted_this_session: HashSet::new(),
            layout_roles: HashMap::new(),
            layout_warning_logged: false,
            debug_input_mode: true,
            debug_border: std::env::var("PRISMX_DEBUG_BORDER").is_ok(),
            status_message: String::new(),
            status_message_last_updated: None,
            plugin_host: PluginHost::new(),
            plugin_favorites: Vec::new(),
            favorite_dock_limit: 3,
            favorite_dock_layout: DockLayout::Vertical,
            favorite_dock_enabled: true,
            last_mouse_click: None,
            settings_focus_index: 0,

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

    pub fn set_selected(&mut self, id: Option<NodeID>) {
        self.selected = id;
        self.last_promoted_root = None;
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

    /// Ensure at least one valid root exists. Promotes the first node if roots
    /// are empty or invalid.
    pub fn ensure_valid_roots(&mut self) {
        self.root_nodes.retain(|id| self.nodes.contains_key(id));
        if self.root_nodes.is_empty() && !self.nodes.is_empty() {
            if let Some((&first_id, _)) = self.nodes.iter().next() {
                if Some(first_id) != self.last_promoted_root {
                    self.root_nodes.push(first_id);
                    self.last_promoted_root = Some(first_id);
                    if self.debug_input_mode {
                        eprintln!(
                            "\u{26a0} root_nodes was empty — promoted Node {} to root",
                            first_id
                        );
                    }
                }
            }
        }
        self.root_nodes.sort_unstable();
        self.root_nodes.dedup();
    }

    /// Ensure nodes have unique positions when auto-arrange is disabled.
    pub fn ensure_grid_positions(&mut self) {
        if self.auto_arrange {
            return;
        }

        let ids: Vec<NodeID> = self.nodes.keys().copied().collect();
        let mut index = 0;
        let (tw, _) = terminal::size().unwrap_or((80, 20));
        let margin = SIBLING_SPACING_X * 2;
        let row_pad = CHILD_SPACING_Y * 2;
        let cols = (tw as i16 / margin.max(1)).max(1) as usize;
        for id in ids {
            if let Some(node) = self.nodes.get_mut(&id) {
                if node.x == 0 && node.y == 0 {
                    node.x = ((index % cols) as i16) * margin + 1;
                    node.y = ((index / cols) as i16) * row_pad + GEMX_HEADER_HEIGHT + 1;
                    index += 1;
                }
            }
        }
    }

    pub fn move_focus_up(&mut self) {
        if let Some(current) = self.selected {
            let order = self.visible_node_order();
            if let Some(pos) = order.iter().position(|&id| id == current) {
                if pos > 0 {
                    self.set_selected(Some(order[pos - 1]));
                }
            }
        }
    }

    pub fn move_focus_down(&mut self) {
        if let Some(current) = self.selected {
            let order = self.visible_node_order();
            if let Some(pos) = order.iter().position(|&id| id == current) {
                if pos + 1 < order.len() {
                    self.set_selected(Some(order[pos + 1]));
                }
            }
        }
    }

    pub fn move_focus_left(&mut self) {
        if let Some(current) = self.selected {
            if let Some(node) = self.nodes.get(&current) {
                if let Some(parent_id) = node.parent {
                    self.set_selected(Some(parent_id));
                }
            }
        }
    }

    pub fn move_focus_right(&mut self) {
        if let Some(current) = self.selected {
            if let Some(node) = self.nodes.get(&current) {
                if !node.children.is_empty() {
                    self.set_selected(Some(node.children[0]));
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
        let Some(parent_id) = self.selected else { return };
        if !self.nodes.contains_key(&parent_id) {
            return;
        }

        let new_id = self.nodes.keys().max().copied().unwrap_or(100) + 1;

        let mut child = Node::new(new_id, "New Child", Some(parent_id));
        if let Some(parent) = self.nodes.get(&parent_id) {
            child.x = parent.x;
            child.y = parent.y + 1;
        }
        if self.debug_input_mode {
            eprintln!(
                "[INSERT] Node {} \u{2192} label=\"{}\", parent={:?}, x={}, y={}, mode={:?}",
                new_id,
                child.label,
                child.parent,
                child.x,
                child.y,
                self.mode
            );
        }

        self.nodes.insert(new_id, child);
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(new_id);
        }

        if !self.root_nodes.contains(&parent_id) {
            self.root_nodes.push(parent_id);
            self.root_nodes.sort_unstable();
            self.root_nodes.dedup();
        }

        self.selected = Some(new_id);
        if !self.auto_arrange {
            self.ensure_grid_positions();
        }
        self.recalculate_roles();
        self.ensure_valid_roots();
    }

    pub fn add_sibling(&mut self) {
        let selected_id = match self.selected {
            Some(id) if self.nodes.contains_key(&id) => id,
            _ => return,
        };
        let parent_id = self.nodes.get(&selected_id).and_then(|n| n.parent);

        let new_id = self.nodes.keys().max().copied().unwrap_or(100) + 1;
        let mut sibling = Node::new(new_id, "New Sibling", parent_id);

        if !self.auto_arrange {
            if let Some(selected) = self.nodes.get(&selected_id) {
                sibling.x = selected.x + SIBLING_SPACING_X;
                sibling.y = selected.y;
            } else {
                sibling.x = (self.nodes.len() as i16 % 5) * SIBLING_SPACING_X;
                sibling.y = GEMX_HEADER_HEIGHT + 2;
            }

            if sibling.x == 0 {
                sibling.x = ((self.nodes.len() as i16) % 5 + 1) * SIBLING_SPACING_X;
            }
        }

        if parent_id.is_none() {
            self.root_nodes.push(new_id);
            self.root_nodes.sort_unstable();
            self.root_nodes.dedup();
        } else if let Some(parent) = self.nodes.get_mut(&parent_id.unwrap()) {
            parent.children.push(new_id);
        }

        if self.debug_input_mode {
            eprintln!(
                "[INSERT] Node {} → label=\"{}\", parent={:?}, x={}, y={}, mode={:?}",
                new_id,
                sibling.label,
                parent_id,
                sibling.x,
                sibling.y,
                self.mode
            );
        }

        self.nodes.insert(new_id, sibling);
        self.selected = Some(new_id);
        if !self.auto_arrange {
            self.ensure_grid_positions();
        }
        self.recalculate_roles();
        self.ensure_valid_roots();
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
            self.set_selected(parent_id.or_else(|| self.root_nodes.first().copied()));
        }
    }

    pub fn toggle_collapse(&mut self) {
        if let Some(node) = self.get_selected_node_mut() {
            node.collapsed = !node.collapsed;
        }
    }

    pub fn execute_spotlight_command(&mut self) {
        let input = self.spotlight_input.trim();
        if !input.is_empty() {
            self.spotlight_history.push_front(input.to_string());
            while self.spotlight_history.len() > 10 {
                self.spotlight_history.pop_back();
            }
        }
        self.spotlight_history_index = None;
        if input == "/start pomodoro" {
            self.plugin_host.start_pomodoro();
        } else if input.starts_with("/countdown ") {
            let rest = &input["/countdown ".len()..];
            let mut parts = rest.splitn(2, ' ');
            let days_part = parts.next().unwrap_or("");
            let label = parts.next().unwrap_or("").to_string();
            if let Some(num) = days_part.strip_prefix('+').and_then(|s| s.strip_suffix('d')) {
                if let Ok(days) = num.parse::<u64>() {
                    self.plugin_host.add_countdown(label, days);
                }
            }
        } else if input.starts_with("/dock_layout=") {
            let layout = &input["/dock_layout=".len()..];
            if layout.eq_ignore_ascii_case("horizontal") {
                self.favorite_dock_layout = DockLayout::Horizontal;
            } else if layout.eq_ignore_ascii_case("vertical") {
                self.favorite_dock_layout = DockLayout::Vertical;
            }
        } else if input.starts_with("/dock_limit=") {
            let value = &input["/dock_limit=".len()..];
            if let Ok(num) = value.parse::<usize>() {
                self.favorite_dock_limit = num.min(5);
            }
        } else if input.starts_with("/dock_enabled=") {
            let value = &input["/dock_enabled=".len()..];
            if let Ok(flag) = value.parse::<bool>() {
                self.favorite_dock_enabled = flag;
            }
        } else {
            match input {
                "/triage" => self.mode = "triage".into(),
                "/zen" => self.mode = "zen".into(),
                "/settings" => self.mode = "settings".into(),
                "/gemx" => self.mode = "gemx".into(),
                "/toggle triage" => self.show_triage = !self.show_triage,
                "/toggle keymap" => self.show_keymap = !self.show_keymap,
                "/toggle spotlight" => self.show_spotlight = !self.show_spotlight,
                "/mode zen" => self.mode = "zen".into(),
                "/mode gemx" => self.mode = "gemx".into(),
                "/arrange" => {
                    self.auto_arrange = true;
                }
                "/clear" => self.zen_buffer = vec![String::new()],
                _ => {}
            }
        }
        self.spotlight_input.clear();
        self.show_spotlight = false;
        self.spotlight_just_opened = false;
    }

    pub fn add_free_node(&mut self) {
        let new_id = self.nodes.keys().max().copied().unwrap_or(100) + 1;
        let mut node = Node::new(new_id, "Free Node", None);
        if !self.auto_arrange {
            node.x = (self.nodes.len() as i16 % 5) * SIBLING_SPACING_X;
            node.y = GEMX_HEADER_HEIGHT + 2;
        }
        self.nodes.insert(new_id, node);
        self.root_nodes.push(new_id);
        self.set_selected(Some(new_id));
        self.recalculate_roles();
    }

    pub fn drill_down(&mut self) {
        if let Some(current) = self.get_selected_node() {
            if !current.children.is_empty() {
                self.set_selected(Some(current.children[0]));
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

    pub fn push_undo(&mut self) {
        self.undo_stack.push(self.nodes.clone());
        self.redo_stack.clear(); // Clear redo on new action
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.nodes.clone());
            self.nodes = prev;
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.nodes.clone());
            self.nodes = next;
        }
    }

    pub fn toggle_snap_grid(&mut self) {
        self.snap_to_grid = !self.snap_to_grid;
    }

    pub fn toggle_debug_border(&mut self) {
        self.debug_border = !self.debug_border;
    }

    pub fn zoom_in(&mut self) {
        self.zoom_scale = (self.zoom_scale + 0.1).min(1.5);
        self.zoom_locked_by_user = true;
        if let Some(id) = self.selected {
            crate::layout::zoom_to_anchor(self, id);
        }
    }

    pub fn zoom_out(&mut self) {
        self.zoom_scale = (self.zoom_scale - 0.1).max(0.5);
        self.zoom_locked_by_user = true;
        if let Some(id) = self.selected {
            crate::layout::zoom_to_anchor(self, id);
        }
    }

    pub fn zoom_reset(&mut self) {
        self.zoom_scale = 1.0;
        self.zoom_locked_by_user = false;
        if let Some(id) = self.selected {
            crate::layout::zoom_to_anchor(self, id);
        }
    }

    pub fn clear_fallback_promotions(&mut self) {
        self.fallback_promoted_this_session.clear();
    }

    pub fn start_drag(&mut self) {
        self.selected_drag_source = self.selected;
    }

    pub fn complete_drag(&mut self, target_id: NodeID) {
        if let Some(source_id) = self.selected_drag_source.take() {
            if source_id == target_id {
                return; // prevent self-parenting
            }

            // Remove from old parent
            if let Some(old_parent_id) = self.nodes.get(&source_id).and_then(|n| n.parent) {
                if let Some(parent) = self.nodes.get_mut(&old_parent_id) {
                    parent.children.retain(|&id| id != source_id);
                }
            } else {
                self.root_nodes.retain(|&id| id != source_id);
            }

            // Reparent
            if let Some(source_node) = self.nodes.get_mut(&source_id) {
                source_node.parent = Some(target_id);
            }

            self.nodes.get_mut(&target_id).map(|t| t.children.push(source_id));
        }
    }

    pub fn start_link(&mut self) {
        self.selected_drag_source = self.selected;
    }

    pub fn complete_link(&mut self, target_id: NodeID) {
        if let Some(source_id) = self.selected_drag_source.take() {
            if source_id != target_id {
                self.link_map.entry(source_id).or_default().push(target_id);
            }
        }
    }

    /// Recalculate parent/child relationships based on node positions.
    /// Nodes directly beneath another (±1 cell) become children of that node.
    /// Nodes on the same row become siblings if that row already has a parent.
    /// Otherwise the node is considered free/root.
    pub fn recalculate_roles(&mut self) {
        use std::collections::HashMap;

        self.clear_fallback_promotions();
        self.layout_roles.clear();

        let ids: Vec<NodeID> = self.nodes.keys().copied().collect();

        // Clear current structure
        for node in self.nodes.values_mut() {
            node.children.clear();
            node.parent = None;
        }
        self.root_nodes.clear();

        // Pass 1: detect direct parent above
        let mut new_parents: HashMap<NodeID, Option<NodeID>> = HashMap::new();
        for &id in &ids {
            let (x, y) = {
                let n = &self.nodes[&id];
                (n.x, n.y)
            };
            let mut parent_id = None;
            for &other in &ids {
                if other == id {
                    continue;
                }
                let op = &self.nodes[&other];
                if (y - (op.y + crate::layout::CHILD_SPACING_Y)).abs() <= 1
                    && (x - op.x).abs() <= 1
                    && y > op.y
                {
                    parent_id = Some(other);
                    break;
                }
            }
            new_parents.insert(id, parent_id);
        }

        // Pass 2: siblings on same row inherit existing parent
        for &id in &ids {
            if new_parents[&id].is_some() {
                continue;
            }
            let y = self.nodes[&id].y;
            for &other in &ids {
                if other == id {
                    continue;
                }
                if (self.nodes[&other].y - y).abs() <= 1 {
                    if let Some(pid) = new_parents[&other] {
                        new_parents.insert(id, Some(pid));
                        break;
                    }
                }
            }
        }

        // Apply structure and lock child positions
        for &id in &ids {
            if let Some(parent_id) = new_parents[&id] {
                let (px, py) = {
                    let p = &self.nodes[&parent_id];
                    (p.x, p.y)
                };
                if let Some(node) = self.nodes.get_mut(&id) {
                    node.parent = Some(parent_id);
                    node.x = px;
                    node.y = py + crate::layout::CHILD_SPACING_Y;
                }
                if let Some(parent) = self.nodes.get_mut(&parent_id) {
                    parent.children.push(id);
                }
            } else {
                if let Some(node) = self.nodes.get_mut(&id) {
                    node.parent = None;
                }
                self.root_nodes.push(id);
            }
        }

        // Deduplicate lists
        self.root_nodes.sort_unstable();
        self.root_nodes.dedup();
        for node in self.nodes.values_mut() {
            node.children.sort_unstable();
            node.children.dedup();
        }

        for &id in &self.root_nodes {
            self.layout_roles.insert(id, LayoutRole::Root);
        }
    }

pub fn get_module_by_index(&self) -> &str {
        match self.module_switcher_index % 4 {
            0 => "gemx",
            1 => "zen",
            2 => "triage",
            3 => "settings",
            _ => "gemx",
        }
    }
}

pub fn register_plugin_favorite(state: &mut AppState, icon: &'static str, command: &'static str) {
    if state.plugin_favorites.len() < 5 {
        state.plugin_favorites.push(FavoriteEntry { icon, command });
    }
}
