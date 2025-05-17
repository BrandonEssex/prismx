use ratatui::{
    backend::Backend,
    layout::Rect,
    Frame,
    widgets::{Block, Borders, Paragraph},
    text::Line,
};

pub fn render_triage_panel<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let tasks = vec![
        Line::from("[ ] Write design draft"),
        Line::from("[x] Fix plugin registry"),
        Line::from("[ ] Review trust seals"),
    ];

    let block = Block::default()
        .title("RoutineForge – Inbox")
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(tasks).block(block);
    f.render_widget(paragraph, area);
}
