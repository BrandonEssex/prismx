use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub prismx_icon: Option<PrismxIconSettings>,
}

#[derive(Debug, Deserialize)]
pub struct PrismxIconSettings {
    pub enabled: bool,
    pub position: Option<String>,
    pub default_mode_color: Option<String>,
    pub show_expand_panel: Option<bool>,
    pub expand_panel_key: Option<String>,
    pub color_modes: Option<HashMap<String, String>>,
    pub animation: Option<HashMap<String, String>>,
    pub beam_map: Option<HashMap<String, String>>,
    pub stealth: Option<StealthSettings>,
    pub shard_array: Option<Vec<ShardEntry>>,
    pub profiles: Option<Vec<IconProfileConfig>>,
}

#[derive(Debug, Deserialize)]
pub struct StealthSettings {
    pub enabled: bool,
    pub override_if: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ShardEntry {
    pub plugin: String,
    pub color: String,
    pub position: String,
}

#[derive(Debug, Deserialize)]
pub struct IconProfileConfig {
    pub name: String,
    pub beam_colors: Option<Vec<String>>,
    pub pulse_enabled: Option<bool>,
}