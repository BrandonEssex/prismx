use crate::beam_color::BeamColor;
use crate::state::{
    AppState,
    ShortcutOverlayMode,
    HeartbeatMode,
};
use crate::theme::fonts::FontStyle;
use super::save_user_settings;
use std::sync::atomic::{AtomicU8, Ordering};

// Theme preset logic
static THEME_INDEX: AtomicU8 = AtomicU8::new(0);
const THEME_PRESETS: [BeamColor; 5] = [
    BeamColor::Prism,
    BeamColor::Infrared,
    BeamColor::Ice,
    BeamColor::Aqua,
    BeamColor::Emerald,
];

pub fn current_theme() -> BeamColor {
    THEME_PRESETS[THEME_INDEX.load(Ordering::Relaxed) as usize]
}

fn toggle_theme(state: &mut AppState) {
    let next = (THEME_INDEX.load(Ordering::Relaxed) + 1) % THEME_PRESETS.len() as u8;
    THEME_INDEX.store(next, Ordering::Relaxed);
    let color = current_theme();
    state.gemx_beam_color = color;
    state.zen_beam_color = color;
    state.triage_beam_color = color;
    state.settings_beam_color = color;
    save_user_settings(state);
}

// Toggle logic
pub struct SettingToggle {
    pub icon: &'static str,
    pub label: &'static str,
    pub is_enabled: fn(&AppState) -> bool,
    pub toggle: fn(&mut AppState),
    pub category: SettingCategory,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SettingCategory {
    Visuals,
    Interaction,
    Modules,
    UX,
}

impl SettingCategory {
    pub fn name(self) -> &'static str {
        match self {
            SettingCategory::Visuals => "Visuals",
            SettingCategory::Interaction => "Interaction",
            SettingCategory::Modules => "Modules",
            SettingCategory::UX => "UX",
        }
    }
}

fn is_debug_mode(s: &AppState) -> bool { s.debug_input_mode }
fn toggle_debug_mode(s: &mut AppState) { s.debug_input_mode = !s.debug_input_mode; save_user_settings(s); }

fn is_allow_empty_nodes(s: &AppState) -> bool { s.debug_allow_empty_nodes }
fn toggle_allow_empty_nodes(s: &mut AppState) {
    s.debug_allow_empty_nodes = !s.debug_allow_empty_nodes;
    save_user_settings(s);
}

fn is_auto_arrange(s: &AppState) -> bool { s.auto_arrange }
fn toggle_auto_arrange(s: &mut AppState) { s.auto_arrange = !s.auto_arrange; save_user_settings(s); }

fn is_zoom_locked(s: &AppState) -> bool { s.zoom_locked_by_user }
fn toggle_zoom_lock(s: &mut AppState) { s.zoom_locked_by_user = !s.zoom_locked_by_user; save_user_settings(s); }

fn is_beamx_panel_visible(s: &AppState) -> bool { s.beamx_panel_visible }
fn toggle_beamx_panel_visibility(s: &mut AppState) { s.beamx_panel_visible = !s.beamx_panel_visible; save_user_settings(s); }

fn is_mindmap_lanes(s: &AppState) -> bool { s.mindmap_lanes }
fn toggle_mindmap_lanes(s: &mut AppState) { s.mindmap_lanes = !s.mindmap_lanes; save_user_settings(s); }

fn is_hierarchy_icons(s: &AppState) -> bool { s.hierarchy_icons }
fn toggle_hierarchy_icons(s: &mut AppState) { s.hierarchy_icons = !s.hierarchy_icons; save_user_settings(s); }

fn toggle_font_style(s: &mut AppState) { s.font_style = s.font_style.next(); save_user_settings(s); }
fn font_style_enabled(_: &AppState) -> bool { true }

fn is_beam_animation(s: &AppState) -> bool { s.beam_animation }
fn toggle_beam_animation(s: &mut AppState) { s.beam_animation = !s.beam_animation; save_user_settings(s); }

fn is_spotlight_auto(s: &AppState) -> bool { s.spotlight_auto_width }
fn toggle_spotlight_auto(s: &mut AppState) { s.spotlight_auto_width = !s.spotlight_auto_width; save_user_settings(s); }

fn is_beam_shimmer(s: &AppState) -> bool { s.beam_shimmer }
fn toggle_beam_shimmer(s: &mut AppState) { s.beam_shimmer = !s.beam_shimmer; save_user_settings(s); }

fn is_focus_trail(s: &AppState) -> bool { s.highlight_focus_branch }
fn toggle_focus_trail(s: &mut AppState) { s.highlight_focus_branch = !s.highlight_focus_branch; save_user_settings(s); }

fn is_zoom_grid(s: &AppState) -> bool { s.zoom_grid }
fn toggle_zoom_grid(s: &mut AppState) { s.zoom_grid = !s.zoom_grid; save_user_settings(s); }

fn is_sticky_notes(s: &AppState) -> bool { s.sticky_notes }
fn toggle_sticky_notes(s: &mut AppState) { s.sticky_notes = !s.sticky_notes; save_user_settings(s); }

fn shortcut_overlay_enabled(s: &AppState) -> bool { matches!(s.shortcut_overlay, ShortcutOverlayMode::Full) }
fn toggle_shortcut_overlay(s: &mut AppState) {
    s.shortcut_overlay = match s.shortcut_overlay {
        ShortcutOverlayMode::Full => ShortcutOverlayMode::Contextual,
        ShortcutOverlayMode::Contextual => ShortcutOverlayMode::Full,
    };
    save_user_settings(s);
}

fn heartbeat_active(s: &AppState) -> bool { !matches!(s.heartbeat_mode, HeartbeatMode::Silent) }
fn toggle_heartbeat(s: &mut AppState) {
    s.heartbeat_mode = match s.heartbeat_mode {
        HeartbeatMode::Pulse => HeartbeatMode::Test,
        HeartbeatMode::Test => HeartbeatMode::Silent,
        HeartbeatMode::Silent => HeartbeatMode::Pulse,
    };
    save_user_settings(s);
}

pub static SETTING_TOGGLES: &[SettingToggle] = &[
    SettingToggle { icon: "🔠", label: "Font Style", is_enabled: font_style_enabled, toggle: toggle_font_style, category: SettingCategory::Visuals },
    SettingToggle { icon: "⚡", label: "Beam Animations", is_enabled: is_beam_animation, toggle: toggle_beam_animation, category: SettingCategory::Visuals },
    SettingToggle { icon: "💫", label: "Beam Shimmer", is_enabled: is_beam_shimmer, toggle: toggle_beam_shimmer, category: SettingCategory::Visuals },
    SettingToggle { icon: "🌀", label: "Focus Trail", is_enabled: is_focus_trail, toggle: toggle_focus_trail, category: SettingCategory::Visuals },
    SettingToggle { icon: "🎨", label: "Theme Preset", is_enabled: |_| true, toggle: toggle_theme, category: SettingCategory::Visuals },
    SettingToggle { icon: "#", label: "Zoom Grid", is_enabled: is_zoom_grid, toggle: toggle_zoom_grid, category: SettingCategory::Visuals },
    SettingToggle { icon: "↔", label: "Spotlight Auto-Width", is_enabled: is_spotlight_auto, toggle: toggle_spotlight_auto, category: SettingCategory::Interaction },
    SettingToggle { icon: "🤖", label: "Auto-Arrange", is_enabled: is_auto_arrange, toggle: toggle_auto_arrange, category: SettingCategory::Interaction },
    SettingToggle { icon: "🔒", label: "Lock Zoom Scale", is_enabled: is_zoom_locked, toggle: toggle_zoom_lock, category: SettingCategory::Interaction },
    SettingToggle { icon: "💠", label: "BeamX Panel", is_enabled: is_beamx_panel_visible, toggle: toggle_beamx_panel_visibility, category: SettingCategory::Modules },
    SettingToggle { icon: "📌", label: "Sticky Notes", is_enabled: is_sticky_notes, toggle: toggle_sticky_notes, category: SettingCategory::Modules },
    SettingToggle { icon: "⌨", label: "Shortcut Overlay", is_enabled: shortcut_overlay_enabled, toggle: toggle_shortcut_overlay, category: SettingCategory::Modules },
    SettingToggle { icon: "✨", label: "Mindmap Lanes", is_enabled: is_mindmap_lanes, toggle: toggle_mindmap_lanes, category: SettingCategory::Modules },
    SettingToggle { icon: "🧠", label: "Hierarchy Icons", is_enabled: is_hierarchy_icons, toggle: toggle_hierarchy_icons, category: SettingCategory::Modules },
    SettingToggle { icon: "🐞", label: "Debug Input Mode", is_enabled: is_debug_mode, toggle: toggle_debug_mode, category: SettingCategory::UX },
    SettingToggle { icon: "⚠", label: "Allow Empty Nodes", is_enabled: is_allow_empty_nodes, toggle: toggle_allow_empty_nodes, category: SettingCategory::UX },
    SettingToggle { icon: "❤️", label: "Heartbeat", is_enabled: heartbeat_active, toggle: toggle_heartbeat, category: SettingCategory::UX },
];

pub fn settings_len() -> usize {
    SETTING_TOGGLES.len()
}
