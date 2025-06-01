// src/settings.rs
use crate::beam_color::BeamColor;
use crate::state::{AppState, LayoutStyle};
use crate::theme::fonts::FontStyle;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UserSettings {
    pub auto_arrange: bool,
    pub debug_input_mode: bool,
    pub debug_allow_empty_nodes: bool,
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
    pub hierarchy_icons: bool,
    pub font_style: FontStyle,
    pub beam_animation: bool,
    pub spotlight_auto_width: bool,
    pub beam_shimmer: bool,
    pub ghost_link_trails: bool,
    pub highlight_focus_branch: bool,
    pub dark_children: bool,
    pub layout_style: LayoutStyle,
    pub zoom_grid: bool,
    pub sticky_notes: bool,
    pub enable_image_drop: bool,
    pub shortcut_overlay: crate::state::ShortcutOverlayMode,
    pub heartbeat_mode: crate::state::HeartbeatMode,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            auto_arrange: true,
            debug_input_mode: true,
            debug_allow_empty_nodes: false,
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
            hierarchy_icons: true,
            font_style: FontStyle::Regular,
            beam_animation: true,
            spotlight_auto_width: false,
            beam_shimmer: true,
            ghost_link_trails: true,
            highlight_focus_branch: false,
            dark_children: false,
            layout_style: LayoutStyle::Compact,
            zoom_grid: false,
            sticky_notes: false,
            enable_image_drop: false,
            shortcut_overlay: crate::state::ShortcutOverlayMode::Full,
            heartbeat_mode: crate::state::HeartbeatMode::Pulse,
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
        debug_allow_empty_nodes: state.debug_allow_empty_nodes,
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
        hierarchy_icons: state.hierarchy_icons,
        font_style: state.font_style,
        beam_animation: state.beam_animation,
        spotlight_auto_width: state.spotlight_auto_width,
        beam_shimmer: state.beam_shimmer,
        ghost_link_trails: state.ghost_link_trails,
        highlight_focus_branch: state.highlight_focus_branch,
        dark_children: state.dark_children,
        layout_style: state.layout_style,
        zoom_grid: state.zoom_grid,
        sticky_notes: state.sticky_notes,
        enable_image_drop: state.enable_image_drop,
        shortcut_overlay: state.shortcut_overlay,
        heartbeat_mode: state.heartbeat_mode,
    };

    if let Ok(serialized) = toml::to_string(&config) {
        let _ = fs::create_dir_all("config");
        let _ = fs::write("config/settings.toml", serialized);
    }
}


pub mod toggle;
pub mod layout;
pub mod render;

pub use toggle::{
    SettingToggle,
    SETTING_TOGGLES,
    settings_len,
    current_theme,
    SETTING_CATEGORIES,
    toggles_for_category,
};
