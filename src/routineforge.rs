use ratatui::{
    backend::Backend,
    layout::Rect,
    Frame,
    widgets::{Block, Borders, Paragraph},
    text::Line,
};
use crate::beamx::{render_full_border, style_for_mode};
use crate::ui::beamx::{BeamX, BeamXStyle, BeamXMode};
use std::time::{SystemTime, UNIX_EPOCH};

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
    render_full_border(f, area, &style, true);
    let tick = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() / 300) as u64;
    let beamx = BeamX {
        tick,
        enabled: true,
        style: BeamXStyle::from(BeamXMode::Triage),
    };
    beamx.render(f, area);
}
