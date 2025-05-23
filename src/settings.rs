use std::time::{SystemTime, UNIX_EPOCH};
use ratatui::{
    backend::Backend,
    layout::{Rect, Alignment},
    Frame,
    widgets::{Block, Borders, Paragraph},
    text::Line,
};
use crate::beamx::{render_full_border, style_for_mode};
use crate::ui::beamx::{BeamX, BeamXStyle, BeamXMode, BeamXAnimationMode};


use crate::state::AppState;

pub fn render_settings<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let style = style_for_mode("settings");
    let debug_label = if state.debug_border { "On" } else { "Off" };
    let lines = vec![
        Line::from("PrismX Settings"),
        Line::from("----------------"),
        Line::from("Theme: theme.toml"),
        Line::from("Layout: Ctrl+Arrow resizable"),
        Line::from("Panels: Toggle w/ Ctrl+D, I, K, Z"),
        Line::from("Commands: /theme, /triage, /plugin, /journal"),
        Line::from(format!("Debug Border: {} (Ctrl+Shift+B)", debug_label)),
    ];

    let block = Block::default()
        .title("Settings")
        .borders(Borders::NONE);
    // Draw title first so the text area can be precisely positioned
    f.render_widget(block, area);

    let inner = Rect::new(area.x + 1, area.y + 1, area.width.saturating_sub(2), area.height.saturating_sub(2));
    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Left);
    f.render_widget(paragraph, inner);
    render_full_border(f, area, &style, true, !state.debug_border);
    let tick = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() / 300) as u64;
    let beamx = BeamX {
        tick,
        enabled: true,
        mode: BeamXMode::Settings,
        style: BeamXStyle::from(BeamXMode::Settings),
        animation: BeamXAnimationMode::PulseEntryRadiate,
    };
    beamx.render(f, area);
}
