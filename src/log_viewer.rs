use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Line, Span},
    style::{Style, Color},
    Frame,
};
use std::fs;

pub fn render_log_viewer(f: &mut Frame<'_>, area: Rect) {
    let contents = fs::read_to_string("logs/qa_runtime.log").unwrap_or_else(|_| "<log unavailable>".into());

    let lines: Vec<Line> = contents
        .lines()
        .rev()
        .take((area.height - 2) as usize)
        .map(|line| Line::from(Span::styled(line, Style::default().fg(Color::Gray))))
        .collect();

    let block = Block::default().title("Log Viewer").borders(Borders::ALL);
    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, area);
}