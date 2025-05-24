use ratatui::{
    backend::Backend,
    layout::{Rect, Alignment},
    Frame,
    widgets::{Block, Borders, Paragraph},
    text::Line,
};
use crate::beamx::{render_beamx, render_full_border, style_for_mode, BeamXStyle};

pub fn render_settings<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let style = style_for_mode("settings");
    let lines = vec![
        Line::from("PrismX Settings"),
        Line::from("----------------"),
        Line::from("Theme: theme.toml"),
        Line::from("Layout: Ctrl+Arrow resizable"),
        Line::from("Panels: Toggle w/ Ctrl+D, I, K, Z"),
        Line::from("Commands: /theme, /triage, /plugin, /journal"),
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
    render_full_border(f, area, &style, true);
    render_beamx(f, area, &style, BeamXStyle::Split);
}
