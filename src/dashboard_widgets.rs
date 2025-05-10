// FINAL FULL FILE DELIVERY
// Filename: /src/dashboard_widgets.rs

use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    text::{Line, Span},
    style::{Style, Color},
    Frame,
};

use chrono::Local;

pub fn render_clock_widget(f: &mut Frame<'_>, area: Rect) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let block = Block::default().title("Clock").borders(Borders::ALL);
    let para = Paragraph::new(vec![Line::from(Span::styled(
        now,
        Style::default().fg(Color::LightYellow),
    ))])
    .block(block);

    f.render_widget(para, area);
}