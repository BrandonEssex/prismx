use ratatui::{
    backend::Backend,
    layout::Rect,
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

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
    render_full_border(f, area, &style);
    render_beamx(f, area, &style, BeamXStyle::Split);
}
