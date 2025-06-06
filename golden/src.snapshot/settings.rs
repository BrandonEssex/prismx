// src/settings.rs
use crate::beam_color::BeamColor;
use crate::state::AppState;
use crate::config::theme::ThemeConfig;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub auto_arrange: bool,
    pub debug_input_mode: bool,
    pub dock_layout: String,
    pub gemx_beam_color: BeamColor,
    pub zen_beam_color: BeamColor,
    pub triage_beam_color: BeamColor,
    pub settings_beam_color: BeamColor,
    pub zen_icon_enabled: bool,
    pub zen_icon_glyph: Option<String>,
    pub beamx_panel_theme: BeamColor,
    pub beamx_panel_visible: bool,
    pub mindmap_lanes: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            auto_arrange: true,
            debug_input_mode: true,
            dock_layout: "vertical".into(),
            gemx_beam_color: BeamColor::Prism,
            zen_beam_color: BeamColor::Prism,
            triage_beam_color: BeamColor::Prism,
            settings_beam_color: BeamColor::Prism,
            zen_icon_enabled: true,
            zen_icon_glyph: None,
            beamx_panel_theme: BeamColor::Prism,
            beamx_panel_visible: crate::state::default_beamx_panel_visible(),
            mindmap_lanes: true,
        }
    }
}

pub fn load_user_settings() -> UserSettings {
    if std::env::var("PRISMX_TEST").is_ok() {
        return UserSettings::default();
    }
    fs::read_to_string("config/settings.toml")
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_user_settings(state: &AppState) {
    let config = UserSettings {
        auto_arrange: state.auto_arrange,
        debug_input_mode: state.debug_input_mode,
        dock_layout: format!("{:?}", state.favorite_dock_layout).to_lowercase(),
        gemx_beam_color: state.gemx_beam_color,
        zen_beam_color: state.zen_beam_color,
        triage_beam_color: state.triage_beam_color,
        settings_beam_color: state.settings_beam_color,
        zen_icon_enabled: state.zen_icon_enabled,
        zen_icon_glyph: state.zen_icon_glyph.clone(),
        beamx_panel_theme: state.beamx_panel_theme,
        beamx_panel_visible: state.beamx_panel_visible,
        mindmap_lanes: state.mindmap_lanes,
    };

    if let Ok(serialized) = toml::to_string(&config) {
        let _ = fs::create_dir_all("config");
        let _ = fs::write("config/settings.toml", serialized);
    }
}

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
