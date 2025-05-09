use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Line, Span},
    style::{Style, Color},
    Frame,
};
use chrono::Local;

pub fn render_clock_widget(f: &mut Frame<'_>, area: Rect) {
    let now = Local::now().format("%H:%M:%S").to_string();

    let para = Paragraph::new(Line::from(vec![Span::styled(
        now,
        Style::default().fg(Color::Green),
    )]))
    .block(Block::default().title("Clock").borders(Borders::ALL));

    f.render_widget(para, area);
}