// FINAL FULL FILE DELIVERY
// Filename: /src/dashboard_widgets.rs

use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Span, Line},
    style::{Style, Color},
    Frame,
};

use chrono::Local;

pub fn render_clock_widget<B: Backend>(f: &mut Frame<'_>, area: Rect) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let para = Paragraph::new(vec![Line::from(Span::styled(
        now,
        Style::default().fg(Color::Yellow),
    ))])
    .block(Block::default().title("Clock").borders(Borders::ALL));

    f.render_widget(para, area);
}