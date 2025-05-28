use crate::beam_color::BeamColor;
use crate::state::AppState;
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
}

fn is_debug_mode(s: &AppState) -> bool { s.debug_input_mode }
fn toggle_debug_mode(s: &mut AppState) { s.debug_input_mode = !s.debug_input_mode; save_user_settings(s); }

fn is_auto_arrange(s: &AppState) -> bool { s.auto_arrange }
fn toggle_auto_arrange(s: &mut AppState) { s.auto_arrange = !s.auto_arrange; save_user_settings(s); }

fn is_zoom_locked(s: &AppState) -> bool { s.zoom_locked_by_user }
fn toggle_zoom_lock(s: &mut AppState) { s.zoom_locked_by_user = !s.zoom_locked_by_user; save_user_settings(s); }

fn is_beamx_panel_visible(s: &AppState) -> bool { s.beamx_panel_visible }
fn toggle_beamx_panel_visibility(s: &mut AppState) { s.beamx_panel_visible = !s.beamx_panel_visible; save_user_settings(s); }

fn is_mindmap_lanes(s: &AppState) -> bool { s.mindmap_lanes }
fn toggle_mindmap_lanes(s: &mut AppState) { s.mindmap_lanes = !s.mindmap_lanes; save_user_settings(s); }

pub static SETTING_TOGGLES: &[SettingToggle] = &[
    SettingToggle { icon: "🐞", label: "Debug Input Mode", is_enabled: is_debug_mode, toggle: toggle_debug_mode },
    SettingToggle { icon: "🤖", label: "Auto-Arrange", is_enabled: is_auto_arrange, toggle: toggle_auto_arrange },
    SettingToggle { icon: "🔒", label: "Lock Zoom Scale", is_enabled: is_zoom_locked, toggle: toggle_zoom_lock },
    SettingToggle { icon: "🎨", label: "Theme Preset", is_enabled: |_| true, toggle: toggle_theme },
    SettingToggle { icon: "💠", label: "BeamX Panel", is_enabled: is_beamx_panel_visible, toggle: toggle_beamx_panel_visibility },
    SettingToggle { icon: "✨", label: "Mindmap Lanes", is_enabled: is_mindmap_lanes, toggle: toggle_mindmap_lanes },
];

pub fn settings_len() -> usize {
    SETTING_TOGGLES.len()
}
