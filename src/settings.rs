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

#[derive(Serialize, Deserialize)]
pub struct UserSettings {
    pub auto_arrange: bool,
    pub debug_input_mode: bool,
    pub dock_layout: String,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            auto_arrange: true,
            debug_input_mode: true,
            dock_layout: "vertical".into(),
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
];

pub fn render_settings<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let lines: Vec<Line> = SETTING_TOGGLES
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let enabled = (t.is_enabled)(state);
            let selected = i == state.settings_focus_index % SETTING_TOGGLES.len();
            let check = if enabled { "[x]" } else { "[ ]" };
            let prefix = if selected { "> " } else { "  " };
            let style = if selected {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            Line::from(vec![
                Span::styled(prefix.to_string(), style),
                Span::styled(format!("{} {}", check, t.label), style),
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
