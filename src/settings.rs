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

use crate::state::{AppState, DockLayout};
use crate::beam_color::BeamColor;

#[derive(Serialize, Deserialize)]
#[serde(default)]
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
    };
    if let Ok(serialized) = toml::to_string(&config) {
        let _ = fs::create_dir_all("config");
        let _ = fs::write("config/settings.toml", serialized);
    }
}

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

fn is_vertical_dock(s: &AppState) -> bool {
    matches!(s.favorite_dock_layout, DockLayout::Vertical)
}
fn toggle_dock_layout(s: &mut AppState) {
    s.favorite_dock_layout = match s.favorite_dock_layout {
        DockLayout::Vertical => DockLayout::Horizontal,
        DockLayout::Horizontal => DockLayout::Vertical,
    };
    s.dock_focus_index = None;
    s.status_message.clear();
    s.status_message_last_updated = None;
    save_user_settings(s);
}

fn toggle_gemx_color(s: &mut AppState) {
    s.cycle_beam_color("gemx");
    save_user_settings(s);
}
fn toggle_zen_color(s: &mut AppState) {
    s.cycle_beam_color("zen");
    save_user_settings(s);
}
fn toggle_triage_color(s: &mut AppState) {
    s.cycle_beam_color("triage");
    save_user_settings(s);
}
fn toggle_settings_color(s: &mut AppState) {
    s.cycle_beam_color("settings");
    save_user_settings(s);
}

pub const SETTING_TOGGLES: &[SettingToggle] = &[
    SettingToggle {
        label: "Auto-Arrange",
        is_enabled: is_auto_arrange,
        toggle: toggle_auto_arrange,
    },
    SettingToggle {
        label: "Debug Mode",
        is_enabled: is_debug_mode,
        toggle: toggle_debug_mode,
    },
    SettingToggle {
        label: "Vertical Dock",
        is_enabled: is_vertical_dock,
        toggle: toggle_dock_layout,
    },
    SettingToggle {
        label: "Gemx Color",
        is_enabled: |_| true,
        toggle: toggle_gemx_color,
    },
    SettingToggle {
        label: "Zen Color",
        is_enabled: |_| true,
        toggle: toggle_zen_color,
    },
    SettingToggle {
        label: "Triage Color",
        is_enabled: |_| true,
        toggle: toggle_triage_color,
    },
    SettingToggle {
        label: "Settings Color",
        is_enabled: |_| true,
        toggle: toggle_settings_color,
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
            if t.label.starts_with("Gemx Color") {
                label = format!("Gemx Color: {}", state.gemx_beam_color);
            } else if t.label.starts_with("Zen Color") {
                label = format!("Zen Color: {}", state.zen_beam_color);
            } else if t.label.starts_with("Triage Color") {
                label = format!("Triage Color: {}", state.triage_beam_color);
            } else if t.label.starts_with("Settings Color") {
                label = format!("Settings Color: {}", state.settings_beam_color);
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
    // Clamp box height so it never overlaps the status bar
    height = height.min(area.height.saturating_sub(1));

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    let block = Block::default().title("Settings").borders(Borders::ALL);
    let content = Paragraph::new(lines).block(block).wrap(Wrap { trim: false });

    f.render_widget(content, Rect::new(x, y, width, height));
}
