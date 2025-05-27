use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Clear,
    Frame,
};

use crate::config::theme::ThemeConfig;
use crate::ui::components::theme::render_theme_editor;

/// Render the settings screen which contains the theme editor.
pub fn render_settings<B: Backend>(f: &mut Frame<B>, area: Rect, cfg: &ThemeConfig) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3)].as_ref())
        .split(area);

    // Clear the area before rendering
    f.render_widget(Clear, area);
    render_theme_editor(f, chunks[0], cfg);
}

let toggles = vec![
    SettingToggle {
        label: "Show BeamX",
        toggle: toggle_beamx,
        icon: "💠",
    },
];

