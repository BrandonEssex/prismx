// src/settings.rs
use crate::beam_color::BeamColor;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use std::fs;

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


pub mod toggle;
pub mod layout;
pub mod render;

pub use toggle::{SettingToggle, SETTING_TOGGLES, settings_len, current_theme};
