use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use serde::{Deserialize, Serialize};
use std::fs;

use crate::state::AppState;
use crate::beam_color::BeamColor;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

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
        }
    }
}

pub fn load_user_settings() -> UserSettings {
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
    };

    if let Ok(serialized) = toml::to_string(&config) {
        let _ = fs::create_dir_all("config");
        let _ = fs::write("config/settings.toml", serialized);
    }
}

static BEAMX_ENABLED: AtomicBool = AtomicBool::new(false);
static THEME_INDEX: AtomicU8 = AtomicU8::new(0);
const THEME_PRESETS: [BeamColor; 5] = [
    BeamColor::Prism,
    BeamColor::Infrared,
    BeamColor::Ice,
    BeamColor::Aqua,
    BeamColor::Emerald,
];

pub struct SettingToggle {
    pub label: &'static str,
    pub is_enabled: fn(&AppState) -> bool,
    pub toggle: fn(&mut AppState),
}

fn is_auto_arrange(s: &AppState) -> bool { s.auto_arrange }
fn toggle_auto_arrange(s: &mut AppState) {
    s.auto_arrange = !s.auto_arrange;
    save_user_settings(s);
}

fn is_debug_mode(s: &AppState) -> bool { s.debug_input_mode }
fn toggle_debug_mode(s: &mut AppState) {
    s.debug_input_mode = !s.debug_input_mode;
    save_user_settings(s);
}


fn is_beamx_enabled(_: &AppState) -> bool {
    BEAMX_ENABLED.load(Ordering::Relaxed)
}
fn toggle_beamx(_: &mut AppState) {
    let new = !BEAMX_ENABLED.load(Ordering::Relaxed);
    BEAMX_ENABLED.store(new, Ordering::Relaxed);
}

fn is_zoom_locked(s: &AppState) -> bool { s.zoom_locked_by_user }
fn toggle_zoom_lock(s: &mut AppState) {
    s.zoom_locked_by_user = !s.zoom_locked_by_user;
    save_user_settings(s);
}

fn theme_label() -> BeamColor {
    THEME_PRESETS[THEME_INDEX.load(Ordering::Relaxed) as usize]
}
fn toggle_theme(state: &mut AppState) {
    let next = (THEME_INDEX.load(Ordering::Relaxed) + 1) % THEME_PRESETS.len() as u8;
    THEME_INDEX.store(next, Ordering::Relaxed);
    let color = theme_label();
    state.gemx_beam_color = color;
    state.zen_beam_color = color;
    state.triage_beam_color = color;
    state.settings_beam_color = color;
    save_user_settings(state);
}

fn is_beamx_panel_visible(s: &AppState) -> bool { s.beamx_panel_visible }
fn toggle_beamx_panel_visibility(s: &mut AppState) {
    s.beamx_panel_visible = !s.beamx_panel_visible;
    save_user_settings(s);
}

fn toggle_beamx_theme(s: &mut AppState) {
    s.cycle_beamx_panel_theme();
    save_user_settings(s);
}

pub const SETTING_TOGGLES: &[SettingToggle] = &[
    SettingToggle {
        label: "BeamX Panel",
        is_enabled: is_beamx_enabled,
        toggle: toggle_beamx,
    },
    SettingToggle {
        label: "Debug Input Mode",
        is_enabled: is_debug_mode,
        toggle: toggle_debug_mode,
    },
    SettingToggle {
        label: "Auto-Arrange",
        is_enabled: is_auto_arrange,
        toggle: toggle_auto_arrange,
    },
    SettingToggle {
        label: "Lock Zoom Scale",
        is_enabled: is_zoom_locked,
        toggle: toggle_zoom_lock,
    },
    SettingToggle {
        label: "Theme Preset",
        is_enabled: |_| true,
        toggle: toggle_theme,
    },
    SettingToggle {
        label: "BeamX Panel",
        is_enabled: is_beamx_panel_visible,
        toggle: toggle_beamx_panel_visibility,
    },
    SettingToggle {
        label: "BeamX Theme",
        is_enabled: |_| true,
        toggle: toggle_beamx_theme,
    },
];

pub const fn settings_len() -> usize {
    SETTING_TOGGLES.len()
}
pub fn render_settings<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let lines: Vec<Line> = SETTING_TOGGLES
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let enabled = (t.is_enabled)(state);
            let selected = i == state.settings_focus_index % SETTING_TOGGLES.len();
            let mut label = t.label.to_string();

            if t.label.starts_with("Theme Preset") {
                label = format!("Theme Preset: {}", theme_label());
            } else if t.label.starts_with("Gemx Color") {
                label = format!("Gemx Color: {}", state.gemx_beam_color);
            } else if t.label.starts_with("Zen Color") {
                label = format!("Zen Color: {}", state.zen_beam_color);
            } else if t.label.starts_with("Triage Color") {
                label = format!("Triage Color: {}", state.triage_beam_color);
            } else if t.label.starts_with("Settings Color") {
                label = format!("Settings Color: {}", state.settings_beam_color);
            } else if t.label.starts_with("BeamX Theme") {
                label = format!("BeamX Theme: {}", state.beamx_panel_theme);
            }

            let check = if enabled { "[x]" } else { "[ ]" };
            let prefix = if selected { "> " } else { "  " };
            let style = if selected {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            Line::from(vec![
                Span::styled(prefix.to_string(), style),
                Span::styled(format!("{} {}", check, label), style),
            ])
        })
        .collect();

    let content_width = lines
        .iter()
        .map(|l| l.width() as u16)
        .max()
        .unwrap_or(0)
        .saturating_add(4);

    let width = content_width.min(area.width);
    let mut height = lines.len() as u16 + 2;
    height = height.min(area.height.saturating_sub(1));

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    let block = Block::default().title("Settings").borders(Borders::ALL);
    let content = Paragraph::new(lines).block(block).wrap(Wrap { trim: false });

    f.render_widget(content, Rect::new(x, y, width, height));
}
