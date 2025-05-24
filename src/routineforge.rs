use ratatui::{
    backend::Backend,
    layout::Rect,
    Frame,
    widgets::{Block, Borders, Paragraph},
    text::Line,
};
use crate::beamx::{render_beam_logo, render_full_border, style_for_mode};

pub fn render_triage_panel<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let style = style_for_mode("triage");
    let tasks = vec![
        Line::from("[ ] Design new node engine"),
        Line::from("[x] Fix dashboard overflow"),
        Line::from("[ ] Write slash command spec"),
    ];

    let block = Block::default()
        .title("RoutineForge – Inbox")
        .borders(Borders::NONE);

    let paragraph = Paragraph::new(tasks).block(block);
    f.render_widget(paragraph, area);
    render_full_border(f, area, &style);
    render_beam_logo(f, area, &style);
}
