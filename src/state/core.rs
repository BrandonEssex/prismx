#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

use std::collections::{HashMap, HashSet, VecDeque};
#[cfg(not(feature = "std"))]
use alloc::collections::VecDeque;
#[cfg(feature = "std")]
use std::time::Instant;
#[cfg(not(feature = "std"))]
use core::time::Duration as Instant;
use crate::node::{Node, NodeID, NodeMap};
use crate::layout::{GEMX_HEADER_HEIGHT, LayoutRole};
use crate::plugin::{loader, PluginHost};
use crate::zen::image::JournalEntry;
pub use crate::zen::state::*;
use crate::modules::triage::sticky::StickyNote;
use crate::ui::drag::DragState;

use crate::hotkeys::load_hotkeys;
use crate::state::config::Config as UserConfig;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq)]
pub struct LayoutSnapshot {
    pub nodes: NodeMap,
    pub root_nodes: Vec<NodeID>,
    pub selected: Option<NodeID>,
    pub selection_trail: VecDeque<(NodeID, Instant)>,
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
pub enum ZenViewMode {
    Write,
    Review,
}

impl Default for ZenViewMode {
    fn default() -> Self { Self::Write }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShortcutOverlayMode {
    Full,
    Contextual,
}

impl Default for ShortcutOverlayMode {
    fn default() -> Self { Self::Full }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeartbeatMode {
    Pulse,
    Test,
    Silent,
}

impl Default for HeartbeatMode {
    fn default() -> Self { Self::Pulse }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutStyle {
    Compact,
    Relaxed,
}

impl Default for LayoutStyle {
    fn default() -> Self { LayoutStyle::Compact }
}

#[derive(Clone)]
pub struct ZenJournalEntry {
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub text: String,
    pub prev_text: Option<String>,
    pub frame: u8,
    pub tags: Vec<String>,
}

#[derive(Clone, Default)]
pub struct DraftState {
    pub text: String,
    pub editing: Option<usize>,
}

pub struct AppState {
    pub mode: String,
    pub zen_buffer: Vec<String>,
    pub nodes: NodeMap,
    pub root_nodes: Vec<NodeID>,
    /// ID to assign to the next created node for fast insertion
    pub next_node_id: NodeID,
    /// Sequence number for auto-generated node labels
    pub next_node_label: u32,
    pub last_promoted_root: Option<NodeID>,
    pub selected: Option<NodeID>,
    pub selection_trail: VecDeque<(NodeID, Instant)>,
    pub spotlight_input: String,
    pub show_spotlight: bool,
    pub prev_show_spotlight: bool,
    pub spotlight_just_opened: bool,
    pub spotlight_animation_frame: u8,
    pub spotlight_history: VecDeque<String>,
    pub spotlight_history_index: Option<usize>,
    pub spotlight_suggestion_index: Option<usize>,
    pub show_logs: bool,
    pub logs_scroll: usize,
    pub logs_filter: String,
    pub show_triage: bool,
    pub show_keymap: bool,
    pub prev_show_keymap: bool,
    pub keymap_animation_frame: u8,
    pub keymap_closing: bool,
    pub module_switcher_open: bool,
    pub module_switcher_index: usize,
    pub module_switcher_animation_frame: u8,
    pub module_switcher_closing: bool,
    pub prev_module_switcher_open: bool,
    pub prev_mode: String,
    pub mindmap_title_frames: u8,
    pub hotkeys: HashMap<String, String>,
    pub scroll_offset: usize,
    pub max_visible_lines: usize,
    pub undo_stack: Vec<LayoutSnapshot>,
    pub redo_stack: Vec<LayoutSnapshot>,
    pub view_stack: Vec<Option<NodeID>>,
    pub selected_drag_source: Option<NodeID>,
    pub link_map: HashMap<NodeID, Vec<NodeID>>,
    pub auto_arrange: bool,
    pub zoom_scale: f32,
    pub zoom_locked_by_user: bool,
    pub scroll_x: i16,
    pub scroll_y: i16,
    pub scroll_target_x: i16,
    pub scroll_target_y: i16,
    pub snap_to_grid: bool,
    pub drawing_root: Option<NodeID>,
    pub dragging: Option<NodeID>,
    pub drag_hover_target: Option<NodeID>,
    pub link_mode: bool,
    pub last_mouse: Option<(i16, i16)>,
    pub fallback_this_frame: bool,
    pub fallback_promoted_this_session: HashSet<NodeID>,
    pub fallback_next_x: i16,
    pub fallback_next_y: i16,
    pub layout_roles: HashMap<NodeID, LayoutRole>,
    pub layout_warning_logged: bool,
    pub layout_fail_count: u8,
    pub layout_key: (usize, u64),
    pub debug_input_mode: bool,
    pub debug_allow_empty_nodes: bool,
    pub debug_border: bool,
    pub debug_overlay: bool,
    pub debug_overlay_sticky: bool,
    pub mindmap_lanes: bool,
    pub hierarchy_icons: bool,
    pub simulate_input_queue: VecDeque<SimInput>,
    pub status_message: String,
    pub status_message_last_updated: Option<Instant>,
    pub zoom_preview_last: Option<Instant>,
    pub plugin_host: PluginHost,
    pub loaded_plugins: Vec<loader::LoadedPlugin>,
    pub plugin_tabs: Vec<crate::plugins::settings::PluginSettingsTab>,
    pub plugin_favorites: Vec<FavoriteEntry>,
    pub favorite_dock_limit: usize,
    pub favorite_dock_layout: DockLayout,
    pub favorite_dock_enabled: bool,
    pub dock_focus_index: Option<usize>,
    pub dock_hover_index: Option<usize>,
    pub last_mouse_click: Option<(u16, u16)>,
    pub settings_focus_index: usize,
    pub settings_selected_tab: usize,
    pub dock_entry_bounds: Vec<(ratatui::layout::Rect, String)>,
    pub settings_toggle_bounds: Vec<(ratatui::layout::Rect, usize)>,
    pub favorite_focus_index: Option<usize>,
    pub dock_pulse_frames: u8,
    pub zen_toolbar_open: bool,
    pub zen_recent_files: Vec<String>,
    pub zen_toolbar_index: usize,
    pub zen_dirty: bool,
    pub zen_last_saved: Option<Instant>,
    pub zen_current_filename: String,
    pub zen_word_count: usize,
    pub zen_current_syntax: ZenSyntax,
    pub zen_theme: ZenTheme,
    pub zen_view_mode: crate::state::ZenViewMode,
    pub zen_layout_mode: crate::state::ZenLayoutMode,
    pub zen_draft: DraftState,
    pub zen_summary_mode: crate::state::ZenSummaryMode,
    pub zen_compose_input: String,
    pub zen_history_index: Option<usize>,
    pub zen_journal_entries: Vec<ZenJournalEntry>,
    pub zen_tag_filter: Option<String>,
    pub triage_tag_filter: Option<String>,
    pub triage_entries: Vec<crate::triage::state::TriageEntry>,
    pub triage_summary: crate::state::view::TriageSummary,
    pub triage_focus_index: usize,
    pub gemx_beam_color: crate::beam_color::BeamColor,
    pub zen_beam_color: crate::beam_color::BeamColor,
    pub triage_beam_color: crate::beam_color::BeamColor,
    pub settings_beam_color: crate::beam_color::BeamColor,
    pub zen_icon_enabled: bool,
    pub zen_icon_glyph: Option<String>,
    pub beamx_panel_theme: crate::beam_color::BeamColor,
    pub beamx_panel_visible: bool,
    pub font_style: crate::theme::fonts::FontStyle,
    pub beam_animation: bool,
    pub spotlight_auto_width: bool,
    pub beam_shimmer: bool,
    pub ghost_link_trails: bool,
    pub highlight_focus_branch: bool,
    pub layout_style: LayoutStyle,
    pub focus_changed_at: Option<Instant>,
    pub zoom_grid: bool,
    pub sticky_notes: bool,
    /// If true, show drop zone in Zen compose view.
    pub enable_image_drop: bool,
    pub sticky_overlay_visible: bool,
    pub sticky_notes_data: Vec<StickyNote>,
    pub sticky_focus: Option<usize>,
    pub sticky_drag: DragState,
    pub shortcut_overlay: ShortcutOverlayMode,
    pub heartbeat_mode: HeartbeatMode,
    pub triage_view_mode: crate::state::TriageViewMode,
    pub plugin_view_mode: crate::state::PluginViewMode,
    pub plugin_tag_filter: crate::state::PluginTagFilter,
    pub plugin_registry_index: usize,
    pub show_plugin_preview: bool,
    pub user_config: UserConfig,
}

impl AppState {
    /// Helper to check if the application is currently in the Triage module.
    pub fn is_triage_mode(&self) -> bool {
        self.mode == "triage"
    }
}

pub fn default_beamx_panel_visible() -> bool {
    #[cfg(target_os = "macos")]
    {
        #[cfg(feature = "std")]
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
            next_node_id: 3,
            next_node_label: 1,
            last_promoted_root: None,
            selected: Some(node_a),
            selection_trail: VecDeque::new(),
            spotlight_input: String::new(),
            show_spotlight: false,
            prev_show_spotlight: false,
            spotlight_just_opened: false,
            spotlight_animation_frame: 0,
            spotlight_history: VecDeque::new(),
            spotlight_history_index: None,
            spotlight_suggestion_index: None,
            show_logs: false,
            logs_scroll: 0,
            logs_filter: String::new(),
            show_triage: false,
            show_keymap: false,
            prev_show_keymap: false,
            keymap_animation_frame: 0,
            keymap_closing: false,
            module_switcher_open: false,
            module_switcher_index: 0,
            module_switcher_animation_frame: 0,
            module_switcher_closing: false,
            prev_module_switcher_open: false,
            prev_mode: "gemx".into(),
            mindmap_title_frames: 0,
            hotkeys: load_hotkeys(),
            scroll_offset: 0,
            max_visible_lines: 20,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            view_stack: Vec::new(),
            selected_drag_source: None,
            link_map: HashMap::new(),
            auto_arrange: true,
            zoom_scale: 1.0,
            zoom_locked_by_user: false,
            scroll_x: 0,
            scroll_y: 0,
            scroll_target_x: 0,
            scroll_target_y: 0,
            snap_to_grid: false,
            drawing_root: None,
            dragging: None,
            drag_hover_target: None,
            link_mode: false,
            last_mouse: None,
            fallback_this_frame: false,
            fallback_promoted_this_session: HashSet::new(),
            fallback_next_x: 6,
            fallback_next_y: GEMX_HEADER_HEIGHT + 2,
            layout_roles: HashMap::new(),
            layout_warning_logged: false,
            layout_fail_count: 0,
            layout_key: (0, 0),
            debug_input_mode: true,
            debug_allow_empty_nodes: false,
            debug_border: {
            #[cfg(feature = "std")]
            { std::env::var("PRISMX_DEBUG_BORDER").is_ok() }
            #[cfg(not(feature = "std"))]
            { false }
        },
            debug_overlay: false,
            debug_overlay_sticky: false,
            mindmap_lanes: true,
            hierarchy_icons: true,
            simulate_input_queue: VecDeque::new(),
            status_message: String::new(),
            status_message_last_updated: None,
            zoom_preview_last: None,
            plugin_host: PluginHost::new(),
            loaded_plugins: Vec::new(),
            plugin_tabs: Vec::new(),
            plugin_favorites: Vec::new(),
            favorite_dock_limit: 3,
            favorite_dock_layout: DockLayout::Horizontal,
            favorite_dock_enabled: true,
            dock_focus_index: None,
            dock_hover_index: None,
            last_mouse_click: None,
            settings_focus_index: 0,
            settings_selected_tab: 0,
            dock_entry_bounds: Vec::new(),
            settings_toggle_bounds: Vec::new(),
            favorite_focus_index: None,
            dock_pulse_frames: 0,
            zen_toolbar_open: false,
            zen_recent_files: vec!["README.md".into()],
            zen_toolbar_index: 0,
            zen_dirty: false,
            zen_last_saved: None,
            zen_current_filename: "draft.prismx".into(),
            zen_word_count: 0,
            zen_current_syntax: ZenSyntax::Markdown,
            zen_theme: ZenTheme::DarkGray,
            zen_view_mode: crate::state::ZenViewMode::default(),
            zen_layout_mode: crate::state::view::ZenLayoutMode::Compose,
            zen_draft: DraftState::default(),
            zen_summary_mode: crate::state::ZenSummaryMode::default(),
            zen_compose_input: String::new(),
            zen_history_index: None,
            zen_journal_entries: Vec::new(),
            zen_tag_filter: None,
            triage_tag_filter: None,
            triage_entries: Vec::new(),
            triage_summary: crate::state::view::TriageSummary::default(),
            triage_focus_index: 0,
            gemx_beam_color: crate::beam_color::BeamColor::Prism,
            zen_beam_color: crate::beam_color::BeamColor::Prism,
            triage_beam_color: crate::beam_color::BeamColor::Prism,
            settings_beam_color: crate::beam_color::BeamColor::Prism,
            zen_icon_enabled: true,
            zen_icon_glyph: None,
            beamx_panel_theme: crate::beam_color::BeamColor::Prism,
            beamx_panel_visible: default_beamx_panel_visible(),
            font_style: crate::theme::fonts::FontStyle::Regular,
            beam_animation: true,
            spotlight_auto_width: false,
            beam_shimmer: true,
            ghost_link_trails: true,
            highlight_focus_branch: false,
            layout_style: LayoutStyle::Compact,
            focus_changed_at: None,
            zoom_grid: false,
            sticky_notes: false,
            enable_image_drop: false,
            sticky_overlay_visible: false,
            sticky_notes_data: Vec::new(),
            sticky_focus: None,
            sticky_drag: DragState::default(),
            shortcut_overlay: ShortcutOverlayMode::Full,
            heartbeat_mode: HeartbeatMode::Pulse,
            triage_view_mode: crate::state::TriageViewMode::default(),
            plugin_view_mode: crate::state::PluginViewMode::default(),
            plugin_tag_filter: crate::state::PluginTagFilter::default(),
            plugin_registry_index: 0,
            show_plugin_preview: false,
            user_config: UserConfig::default(),
        };

        let config = crate::settings::load_user_settings();
        state.auto_arrange = config.auto_arrange;
        state.debug_input_mode = config.debug_input_mode;
        state.debug_allow_empty_nodes = config.debug_allow_empty_nodes;
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
        state.mindmap_lanes = config.mindmap_lanes;
        state.hierarchy_icons = config.hierarchy_icons;
        state.font_style = config.font_style;
        state.beam_animation = config.beam_animation;
        state.spotlight_auto_width = config.spotlight_auto_width;
        state.beam_shimmer = config.beam_shimmer;
        state.ghost_link_trails = config.ghost_link_trails;
        state.highlight_focus_branch = config.highlight_focus_branch;
        state.layout_style = config.layout_style;
        state.zoom_grid = config.zoom_grid;
        state.sticky_notes = config.sticky_notes;
        state.enable_image_drop = config.enable_image_drop;
        state.shortcut_overlay = config.shortcut_overlay;
        state.heartbeat_mode = config.heartbeat_mode;

        let user_cfg = UserConfig::load();
        state.user_config = user_cfg.clone();
        state.show_logs = user_cfg.ui.show_logs;
        state.auto_arrange = user_cfg.behavior.auto_arrange;

        for node in state.nodes.values_mut() {
            if node.label.starts_with("[F]") {
                node.label = node.label.replacen("[F] ", "", 1);
            }
        }

        state.update_zen_word_count();
        state.load_today_journal();
        crate::modules::triage::feed::sync_from_zen(&mut state);
        state.audit_node_graph();

        if let Some(layout) = crate::config::load_config().layout {
            crate::state::serialize::apply(&mut state, layout);
        }

        state.loaded_plugins = {
            #[cfg(feature = "std")]
            { loader::discover_plugins(std::path::Path::new("plugins")) }
            #[cfg(not(feature = "std"))]
            { Vec::new() }
        };

        // Retrieve any plugin-defined settings tabs registered during plugin initialization
        state.plugin_tabs = crate::plugins::registry::plugin_tabs();

        state.scroll_target_x = state.scroll_x;
        state.scroll_target_y = state.scroll_y;

        state
    }
}

