use std::collections::{HashMap, HashSet, VecDeque};
use crate::node::{Node, NodeID, NodeMap};
use crate::layout::{ SIBLING_SPACING_X, CHILD_SPACING_Y, GEMX_HEADER_HEIGHT, LayoutRole };
use crossterm::terminal;
use crate::plugin::PluginHost;

const UNDO_LIMIT: usize = 50;

mod hotkeys;
pub use hotkeys::*;

#[derive(Clone, PartialEq)]
pub struct LayoutSnapshot {
    pub nodes: NodeMap,
    pub root_nodes: Vec<NodeID>,
    pub selected: Option<NodeID>,
}



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

#[derive(Debug)]
pub enum SimInput {
    Enter,
    Tab,
    Delete,
    Drill,
    Pop,
    Undo,
    Redo,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ZenSyntax {
    Markdown,
    Rust,
    Shell,
    Yaml,
    Json,
    None,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ZenTheme {
    DarkGray,
    Light,
    HighContrast,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ZenJournalView {
    Compose,
    Review,
}

#[derive(Clone)]
pub struct ZenJournalEntry {
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub text: String,
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
    pub undo_stack: Vec<LayoutSnapshot>,
    pub redo_stack: Vec<LayoutSnapshot>,
    pub view_stack: Vec<Option<NodeID>>,
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
    pub fallback_next_x: i16,
    pub fallback_next_y: i16,
    pub layout_roles: HashMap<NodeID, LayoutRole>,
    pub layout_warning_logged: bool,
    pub debug_input_mode: bool,
    pub debug_border: bool,
    pub simulate_input_queue: VecDeque<SimInput>,
    pub status_message: String,
    pub status_message_last_updated: Option<std::time::Instant>,
    pub plugin_host: PluginHost,
    pub plugin_favorites: Vec<FavoriteEntry>,
    pub favorite_dock_limit: usize,
    pub favorite_dock_layout: DockLayout,
    pub favorite_dock_enabled: bool,
    pub dock_focus_index: Option<usize>,
    pub last_mouse_click: Option<(u16, u16)>,
    pub settings_focus_index: usize,
    pub dock_entry_bounds: Vec<(ratatui::layout::Rect, String)>,
    pub favorite_focus_index: Option<usize>,
    pub zen_toolbar_open: bool,
    pub zen_recent_files: Vec<String>,
    pub zen_toolbar_index: usize,
    pub zen_dirty: bool,
    pub zen_last_saved: Option<std::time::Instant>,
    pub zen_current_filename: String,
    pub zen_word_count: usize,
    pub zen_current_syntax: ZenSyntax,
    pub zen_theme: ZenTheme,
    pub zen_journal_view: ZenJournalView,
    pub zen_compose_input: String,
    pub zen_journal_entries: Vec<ZenJournalEntry>,
    pub spotlight_results: Vec<crate::spotlight::SpotlightResult>,
    #[allow(clippy::option_option)]
    pub spotlight_task: Option<crate::spotlight_task::SpotlightTask>,


}

impl Default for AppState {
    fn default() -> Self {
        let mut nodes = NodeMap::new();
        let node_a = 1;
        let node_b = 2;

        nodes.insert(node_a, Node::new(node_a, "Node A", None));
        nodes.insert(node_b, Node::new(node_b, "Node B", None));

        let mut state = Self {
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
            view_stack: Vec::new(),
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
            fallback_next_x: 6,
            fallback_next_y: GEMX_HEADER_HEIGHT + 2,
            layout_roles: HashMap::new(),
            layout_warning_logged: false,
            debug_input_mode: true,
            debug_border: std::env::var("PRISMX_DEBUG_BORDER").is_ok(),
            simulate_input_queue: VecDeque::new(),
            status_message: String::new(),
            status_message_last_updated: None,
            plugin_host: PluginHost::new(),
            plugin_favorites: Vec::new(),
            favorite_dock_limit: 3,
            favorite_dock_layout: DockLayout::Vertical,
            favorite_dock_enabled: true,
            dock_focus_index: None,
            last_mouse_click: None,
            settings_focus_index: 0,
            dock_entry_bounds: Vec::new(),
            favorite_focus_index: None,
            zen_toolbar_open: false,
            zen_recent_files: vec!["README.md".into()],
            zen_toolbar_index: 0,
            zen_dirty: false,
            zen_last_saved: None,
            zen_current_filename: "draft.prismx".into(),
            zen_word_count: 0,
            zen_current_syntax: ZenSyntax::Markdown,
            zen_theme: ZenTheme::DarkGray,
            zen_journal_view: ZenJournalView::Compose,
            zen_compose_input: String::new(),
            zen_journal_entries: Vec::new(),
            spotlight_results: Vec::new(),
            spotlight_task: None,

        };

        let config = crate::settings::load_user_settings();
        state.auto_arrange = config.auto_arrange;
        state.debug_input_mode = config.debug_input_mode;
        state.favorite_dock_layout = match config.dock_layout.as_str() {
            "horizontal" => DockLayout::Horizontal,
            _ => DockLayout::Vertical,
        };

        for node in state.nodes.values_mut() {
            if node.label.starts_with("[F]") {
                node.label = node.label.replacen("[F] ", "", 1);
            }
        }

        state.update_zen_word_count();
        state.load_today_journal();

        state
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

    pub fn start_spotlight_index(&mut self) {
        if let Some(task) = self.spotlight_task.take() {
            task.cancel();
        }
        let nodes = self.nodes.clone();
        let query = self.spotlight_input.clone();
        let task = crate::spotlight_task::SpotlightTask::spawn(nodes, query);
        self.spotlight_results.clear();
        self.spotlight_task = Some(task);
    }

    pub fn poll_spotlight_results(&mut self) {
        if let Some(task) = &self.spotlight_task {
            if let Ok(results) = task.result_rx.try_recv() {
                self.spotlight_results = results;
                self.spotlight_task.take();
            }
        }
    }

    pub fn dock_focus_prev(&mut self) {
        let len = self.favorite_entries().len();
        if len == 0 {
            self.dock_focus_index = None;
            return;
        }
        self.dock_focus_index = Some(match self.dock_focus_index.unwrap_or(0) {
            0 => len - 1,
            i => i - 1,
        });
        if let Some(idx) = self.dock_focus_index {
            if let Some(entry) = self.favorite_entries().get(idx) {
                self.status_message = entry.command.to_string();
                self.status_message_last_updated = Some(std::time::Instant::now());
            }
        }
    }

    pub fn dock_focus_next(&mut self) {
        let len = self.favorite_entries().len();
        if len == 0 {
            self.dock_focus_index = None;
            return;
        }
        self.dock_focus_index = Some(match self.dock_focus_index.unwrap_or(len - 1) {
            i if i + 1 >= len => 0,
            i => i + 1,
        });
        if let Some(idx) = self.dock_focus_index {
            if let Some(entry) = self.favorite_entries().get(idx) {
                self.status_message = entry.command.to_string();
                self.status_message_last_updated = Some(std::time::Instant::now());
            }
        }
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

        if parent_id == new_id {
            eprintln!("❌ Invalid insert: node cannot parent itself.");
            return;
        }

        if let Some(parent) = self.nodes.get(&parent_id) {
            if parent.children.contains(&parent_id) {
                eprintln!("❌ Cycle detected: parent already linked to self.");
                return;
            }
        }

        let mut child = Node::new(new_id, "New Child", Some(parent_id));
        if self.auto_arrange {
            if let Some(parent) = self.nodes.get(&parent_id) {
                child.x = parent.x;
                child.y = parent.y + 1;
            }
        } else {
            let base_x = 6 + ((self.nodes.len() as i16) % 10) * SIBLING_SPACING_X;
            let base_y =
                GEMX_HEADER_HEIGHT + 2 + ((self.nodes.len() as i16) / 10) * CHILD_SPACING_Y;
            child.x = base_x;
            child.y = base_y;
        }

        self.nodes.insert(new_id, child);
        crate::log_debug!(self, "Inserted node {} → parent {:?}", new_id, parent_id);
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


        self.nodes.insert(new_id, sibling);
        crate::log_debug!(self, "Inserted node {} → parent {:?}", new_id, parent_id);
        self.selected = Some(new_id);
        if !self.auto_arrange {
            self.ensure_grid_positions();
        }
        self.recalculate_roles();
        self.ensure_valid_roots();
    }

    pub fn exit_spotlight(&mut self) {
        self.spotlight_input.clear();
        self.show_spotlight = false;
        self.spotlight_just_opened = false;
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
        let command = self.spotlight_input.clone();
        let input = command.trim();
        crate::log_debug!(self, "Executing spotlight: {}", input);
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
            self.dock_focus_index = None;
            self.status_message.clear();
            self.status_message_last_updated = None;
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
            if !self.favorite_dock_enabled {
                self.dock_focus_index = None;
                self.status_message.clear();
                self.status_message_last_updated = None;
            }
        } else if input.starts_with("/simulate") {
            for token in input.split_whitespace().skip(1) {
                match token.to_lowercase().as_str() {
                    "enter" => self.simulate_input_queue.push_back(SimInput::Enter),
                    "tab" => self.simulate_input_queue.push_back(SimInput::Tab),
                    "delete" => self.simulate_input_queue.push_back(SimInput::Delete),
                    "drill" => self.simulate_input_queue.push_back(SimInput::Drill),
                    "pop" => self.simulate_input_queue.push_back(SimInput::Pop),
                    "undo" => self.simulate_input_queue.push_back(SimInput::Undo),
                    "redo" => self.simulate_input_queue.push_back(SimInput::Redo),
                    _ => continue,
                }
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
            "/arrange" => self.auto_arrange = true,
            "/undo" => self.undo(),
            "/redo" => self.redo(),
            "/toolbar" => {
                self.zen_toolbar_open = !self.zen_toolbar_open;
                self.zen_toolbar_index = 0;
            }
            _ if input.starts_with("/open ") => {
                let path = input.trim_start_matches("/open ").trim();
                if !path.is_empty() {
                    self.open_zen_file(path);
                    self.mode = "zen".into();
                }
            }
            "/clear" => self.zen_buffer = vec![String::new()],
            _ => {}
            }
        }
        self.exit_spotlight();
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

    pub fn open_zen_file(&mut self, path: &str) {
        if let Ok(content) = std::fs::read_to_string(path) {
            self.zen_buffer = content.lines().map(|l| l.to_string()).collect();
            self.zen_buffer.push(String::new());
            self.zen_current_filename = path.to_string();
            self.zen_current_syntax = Self::syntax_from_extension(path);
            self.update_zen_word_count();
            self.zen_dirty = false;
            self.add_recent_file(path);
            self.zen_current_filename = path.to_string();
            self.zen_dirty = false;
            self.zen_last_saved = Some(std::time::Instant::now());
        }
    }

    pub fn save_zen_file(&mut self, path: &str) {
        use std::io::Write;
        if let Some(parent) = std::path::Path::new(path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(mut file) = std::fs::File::create(path) {
            let _ = file.write_all(self.zen_buffer.join("\n").as_bytes());
            self.add_recent_file(path);
            self.zen_current_filename = path.to_string();
            self.zen_dirty = false;
            self.zen_last_saved = Some(std::time::Instant::now());
        }
    }

    pub fn update_zen_word_count(&mut self) {
        let text = self.zen_buffer.join(" ");
        self.zen_word_count = text.split_whitespace().count();
        crate::log_debug!(self, "Word count updated: {}", self.zen_word_count);
    }

    pub fn add_recent_file(&mut self, path: &str) {
        if let Some(pos) = self.zen_recent_files.iter().position(|p| p == path) {
            self.zen_recent_files.remove(pos);
        }
        self.zen_recent_files.insert(0, path.to_string());
        while self.zen_recent_files.len() > 5 {
            self.zen_recent_files.pop();
        }
    }

    pub fn zen_toolbar_len(&self) -> usize {
        3 + self.zen_recent_files.len()
    }

    pub fn zen_toolbar_handle_key(&mut self, key: crossterm::event::KeyCode) {
        let len = self.zen_toolbar_len();
        match key {
            crossterm::event::KeyCode::Up => {
                if self.zen_toolbar_index == 0 {
                    self.zen_toolbar_index = len.saturating_sub(1);
                } else {
                    self.zen_toolbar_index -= 1;
                }
            }
            crossterm::event::KeyCode::Down => {
                self.zen_toolbar_index = (self.zen_toolbar_index + 1) % len;
            }
            crossterm::event::KeyCode::Enter => {
                match self.zen_toolbar_index {
                    0 => {
                        self.zen_buffer = vec![String::new()];
                        self.zen_current_filename = "Untitled".into();
                        self.update_zen_word_count();
                        self.zen_dirty = false;
                    }
                    1 => {
                        if let Some(path) = self.zen_recent_files.first().cloned() {
                            self.open_zen_file(&path);
                        }
                    }
                    2 => {
                        if let Some(path) = self.zen_recent_files.first().cloned() {
                            self.save_zen_file(&path);
                        }
                    }
                    idx => {
                        if let Some(path) = self.zen_recent_files.get(idx - 3).cloned() {
                            self.open_zen_file(&path);
                        }
                    }
                }
                self.zen_toolbar_open = false;
            }
            _ => {}
        }
    }

    pub fn auto_save_zen(&mut self) {
        if self.zen_dirty {
            let should_save = self
                .zen_last_saved
                .map_or(true, |t| t.elapsed().as_secs() > 10);
            if should_save {
                let _ = std::fs::write(
                    &self.zen_current_filename,
                    self.zen_buffer.join("\n"),
                );
                self.zen_last_saved = Some(std::time::Instant::now());
                self.zen_dirty = false;
            }
        }
    }

    pub fn load_today_journal(&mut self) {
        use std::fs;
        let path = format!("journals/{}.prismx", chrono::Local::now().format("%Y-%m-%d"));
        if let Ok(content) = fs::read_to_string(&path) {
            self.zen_journal_entries = content
                .lines()
                .filter_map(|line| {
                    let (ts, text) = line.split_once('|')?;
                    chrono::DateTime::parse_from_rfc3339(ts)
                        .ok()
                        .map(|dt| ZenJournalEntry { timestamp: dt.with_timezone(&chrono::Local), text: text.to_string() })
                })
                .collect();
        }
    }

    pub fn append_journal_entry(&mut self, entry: &ZenJournalEntry) {
        use std::fs::{OpenOptions};
        use std::io::Write;
        let dir = "journals";
        let _ = std::fs::create_dir_all(dir);
        let path = format!("{}/{}.prismx", dir, chrono::Local::now().format("%Y-%m-%d"));
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&path) {
            let _ = writeln!(file, "{}|{}", entry.timestamp.to_rfc3339(), entry.text);
        }
    }

    pub fn push_undo(&mut self) {
        let snap = LayoutSnapshot {
            nodes: self.nodes.clone(),
            root_nodes: self.root_nodes.clone(),
            selected: self.selected,
        };
        if self.undo_stack.last().map(|s| s == &snap).unwrap_or(false) {
            return;
        }
        self.undo_stack.push(snap);
        if self.undo_stack.len() > UNDO_LIMIT {
            let excess = self.undo_stack.len() - UNDO_LIMIT;
            self.undo_stack.drain(0..excess);
        }
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            crate::log_debug!(self, "UNDO triggered: stack now = {}", self.undo_stack.len());
            let current = LayoutSnapshot {
                nodes: self.nodes.clone(),
                root_nodes: self.root_nodes.clone(),
                selected: self.selected,
            };
            self.redo_stack.push(current);
            self.nodes = prev.nodes;
            self.root_nodes = prev.root_nodes;
            self.selected = prev.selected;
            self.recalculate_roles();
            self.ensure_valid_roots();
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            crate::log_debug!(self, "REDO triggered: stack now = {}", self.redo_stack.len());
            let current = LayoutSnapshot {
                nodes: self.nodes.clone(),
                root_nodes: self.root_nodes.clone(),
                selected: self.selected,
            };
            self.undo_stack.push(current);
            self.nodes = next.nodes;
            self.root_nodes = next.root_nodes;
            self.selected = next.selected;
            self.recalculate_roles();
            self.ensure_valid_roots();
        }
    }

    pub fn toggle_snap_grid(&mut self) {
        self.snap_to_grid = !self.snap_to_grid;
    }

    pub fn toggle_debug_border(&mut self) {
        self.debug_border = !self.debug_border;
    }

    pub fn cycle_zen_theme(&mut self) {
        self.zen_theme = match self.zen_theme {
            ZenTheme::DarkGray => ZenTheme::Light,
            ZenTheme::Light => ZenTheme::HighContrast,
            ZenTheme::HighContrast => ZenTheme::DarkGray,
        };
    }

    pub fn syntax_from_extension(path: &str) -> ZenSyntax {
        let ext = std::path::Path::new(path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        match ext.as_str() {
            "md" | "markdown" => ZenSyntax::Markdown,
            "rs" => ZenSyntax::Rust,
            "sh" | "bash" => ZenSyntax::Shell,
            "yml" | "yaml" => ZenSyntax::Yaml,
            "json" => ZenSyntax::Json,
            _ => ZenSyntax::None,
        }
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

    pub fn drill_selected(&mut self) {
        if let Some(id) = self.selected {
            if self.nodes.contains_key(&id) {
                self.view_stack.push(self.drawing_root.take());
                self.drawing_root = Some(id);
                self.scroll_x = 0;
                self.scroll_y = 0;
                self.zoom_scale = 1.0;
            } else {
                eprintln!("❌ Drill failed: selected node not found.");
            }
        }
        self.fallback_this_frame = false;
        self.clear_fallback_promotions();
    }

    pub fn pop_stack(&mut self) {
        if let Some(prev) = self.view_stack.pop() {
            if let Some(id) = prev {
                if self.nodes.contains_key(&id) {
                    self.drawing_root = Some(id);
                    self.scroll_x = 0;
                    self.scroll_y = 0;
                    self.zoom_scale = 1.0;
                    self.fallback_this_frame = false;
                    self.clear_fallback_promotions();
                    return;
                }
            }
        }

        self.drawing_root = None;
        self.scroll_x = 0;
        self.scroll_y = 0;
        self.zoom_scale = 1.0;
        self.fallback_this_frame = false;
        self.clear_fallback_promotions();
    }

    pub fn clear_fallback_promotions(&mut self) {
        self.fallback_promoted_this_session.clear();
        self.fallback_next_x = 6;
        self.fallback_next_y = GEMX_HEADER_HEIGHT + 2;
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

    pub fn get_module_by_index(&self) -> &str {
        match self.module_switcher_index % 4 {
            0 => "gemx",
            1 => "zen",
            2 => "triage",
            3 => "settings",
            _ => "gemx",
        }
    }

    pub fn favorite_entries(&self) -> Vec<FavoriteEntry> {
        let default_favorites = [
            ("⚙️", "/settings"),
            ("📬", "/triage"),
            ("💭", "/gemx"),
            ("🧘", "/zen"),
            ("🔍", "/spotlight"),
        ];

        let limit = self.favorite_dock_limit.min(5);
        let mut all: Vec<FavoriteEntry> = self
            .plugin_favorites
            .iter()
            .cloned()
            .chain(
                default_favorites
                    .iter()
                    .map(|&(icon, cmd)| FavoriteEntry { icon, command: cmd }),
            )
            .take(limit)
            .collect();

        if self.mode == "gemx" && all.len() >= 3 {
            all[2].icon = "💬";
        }
        if (self.mode == "triage" || self.show_triage) && all.len() >= 2 {
            all[1].icon = "📫";
        }
        all
    }

    pub fn trigger_favorite(&mut self, index: usize) {
        let entries = self.favorite_entries();
        if let Some(entry) = entries.get(index) {
            self.spotlight_input = entry.command.to_string();
            self.show_spotlight = true;
            self.favorite_focus_index = Some(index);
            self.status_message = entry.command.to_string();
            self.status_message_last_updated = Some(std::time::Instant::now());
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
                if y > op.y
                    && (y - op.y) <= crate::layout::CHILD_SPACING_Y + 1
                    && (x - op.x).abs() <= 1
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
                if parent_id == id {
                    continue;
                }
                crate::log_debug!(self, "Assigning parent {:?} \u{2192} {}", parent_id, id);
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
}

// Outside impl block:
pub fn register_plugin_favorite(state: &mut AppState, icon: &'static str, command: &'static str) {
    if state.plugin_favorites.len() < 5 {
        state.plugin_favorites.push(FavoriteEntry { icon, command });
    }
}
