use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;
use crate::node::{Node, NodeID, NodeMap};
use crate::layout::{GEMX_HEADER_HEIGHT, LayoutRole};
use crate::plugin::PluginHost;

use super::hotkeys::load_default_hotkeys;

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
    pub spotlight_suggestion_index: Option<usize>,
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
    pub link_mode: bool,
    pub last_mouse: Option<(i16, i16)>,
    pub fallback_this_frame: bool,
    pub fallback_promoted_this_session: HashSet<NodeID>,
    pub fallback_next_x: i16,
    pub fallback_next_y: i16,
    pub layout_roles: HashMap<NodeID, LayoutRole>,
    pub layout_warning_logged: bool,
    pub layout_fail_count: u8,
    pub debug_input_mode: bool,
    pub debug_border: bool,
    pub simulate_input_queue: VecDeque<SimInput>,
    pub status_message: String,
    pub status_message_last_updated: Option<std::time::Instant>,
    pub zoom_preview_last: Option<Instant>,
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
    pub zen_view_mode: crate::state::ZenViewMode,
    pub zen_compose_input: String,
    pub zen_journal_entries: Vec<ZenJournalEntry>,
    pub zen_tag_filter: Option<String>,
    pub triage_entries: Vec<crate::triage::logic::TriageEntry>,
    pub gemx_beam_color: crate::beam_color::BeamColor,
    pub zen_beam_color: crate::beam_color::BeamColor,
    pub triage_beam_color: crate::beam_color::BeamColor,
    pub settings_beam_color: crate::beam_color::BeamColor,
    pub zen_icon_enabled: bool,
    pub zen_icon_glyph: Option<String>,
    pub beamx_panel_theme: crate::beam_color::BeamColor,
    pub beamx_panel_visible: bool,
    pub triage_view_mode: crate::state::TriageViewMode,
}

pub fn default_beamx_panel_visible() -> bool {
    #[cfg(target_os = "macos")]
    {
        if let Ok(term) = std::env::var("TERM_PROGRAM") {
            if term.to_lowercase().contains("iterm") {
                return true;
            }
        }
    }
    false
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
            spotlight_suggestion_index: None,
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
            link_mode: false,
            last_mouse: None,
            fallback_this_frame: false,
            fallback_promoted_this_session: HashSet::new(),
            fallback_next_x: 6,
            fallback_next_y: GEMX_HEADER_HEIGHT + 2,
            layout_roles: HashMap::new(),
            layout_warning_logged: false,
            layout_fail_count: 0,
            debug_input_mode: true,
            debug_border: std::env::var("PRISMX_DEBUG_BORDER").is_ok(),
            simulate_input_queue: VecDeque::new(),
            status_message: String::new(),
            status_message_last_updated: None,
            zoom_preview_last: None,
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
            zen_view_mode: crate::state::ZenViewMode::default(),
            zen_compose_input: String::new(),
            zen_journal_entries: Vec::new(),
            zen_tag_filter: None,
            triage_entries: Vec::new(),
            gemx_beam_color: crate::beam_color::BeamColor::Prism,
            zen_beam_color: crate::beam_color::BeamColor::Prism,
            triage_beam_color: crate::beam_color::BeamColor::Prism,
            settings_beam_color: crate::beam_color::BeamColor::Prism,
            zen_icon_enabled: true,
            zen_icon_glyph: None,
            beamx_panel_theme: crate::beam_color::BeamColor::Prism,
            beamx_panel_visible: default_beamx_panel_visible(),
            triage_view_mode: crate::state::TriageViewMode::default(),
        };

        let config = crate::settings::load_user_settings();
        state.auto_arrange = config.auto_arrange;
        state.debug_input_mode = config.debug_input_mode;
        state.favorite_dock_layout = match config.dock_layout.as_str() {
            "horizontal" => DockLayout::Horizontal,
            _ => DockLayout::Vertical,
        };
        state.gemx_beam_color = config.gemx_beam_color;
        state.zen_beam_color = config.zen_beam_color;
        state.triage_beam_color = config.triage_beam_color;
        state.settings_beam_color = config.settings_beam_color;
        state.zen_icon_enabled = config.zen_icon_enabled;
        state.zen_icon_glyph = config.zen_icon_glyph.clone();
        state.beamx_panel_theme = config.beamx_panel_theme;
        state.beamx_panel_visible = config.beamx_panel_visible;

        for node in state.nodes.values_mut() {
            if node.label.starts_with("[F]") {
                node.label = node.label.replacen("[F] ", "", 1);
            }
        }

        state.update_zen_word_count();
        state.load_today_journal();
        state.audit_node_graph();

        if let Some(layout) = crate::config::load_config().layout {
            crate::state::serialize::apply(&mut state, layout);
        }

        state
    }
}

