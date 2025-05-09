use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Line, Span},
    style::{Style, Color},
    Frame,
};
use std::fs;

pub fn render_log_viewer<B>(frame: &mut Frame<B>, area: Rect)
where
    B: ratatui::backend::Backend,
{
    let contents = fs::read_to_string("logs/qa_runtime.log").unwrap_or_else(|_| "<log unavailable>".into());
    let lines: Vec<Line> = contents
        .lines()
        .rev()
        .take((area.height - 2).into())
        .map(|l| Line::from(Span::styled(l, Style::default().fg(Color::Gray))))
        .collect();

    let block = Block::default().title("Log Viewer").borders(Borders::ALL);
    let para = Paragraph::new(lines).block(block);
    frame.render_widget(para, area);
}