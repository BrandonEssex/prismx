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
        Line::from("Theme: loaded from theme.toml"),
        Line::from("Layout: resizable, Ctrl+Arrow"),
        Line::from("Toggle Keys: Dashboard, Triage, Spotlight"),
        Line::from("Slash Commands: /theme, /plugin, /triage, /journal"),
    ];

    let block = Block::default()
        .title("Settings")
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}
