use ratatui::{
    backend::Backend,
    layout::Rect,
    Frame,
    widgets::{Block, Borders, Paragraph},
    text::Line,
};

pub fn render_triage_panel<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let tasks = vec![
        Line::from("[ ] Design new node engine"),
        Line::from("[x] Fix dashboard overflow"),
        Line::from("[ ] Write slash command spec"),
    ];

    let block = Block::default()
        .title("RoutineForge – Inbox")
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(tasks).block(block);
    f.render_widget(paragraph, area);
}
