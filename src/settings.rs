use ratatui::{
    backend::Backend,
    layout::Rect,
    Frame,
    widgets::{Block, Borders, Paragraph},
    text::Line,
};

pub fn render_settings_panel<B: Backend>(f: &mut Frame<B>, area: Rect) {
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
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}
