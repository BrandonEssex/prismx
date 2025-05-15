use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Style, Color},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use chrono::Local;

pub fn render_clock_widget<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let now = Local::now().format("%H:%M:%S").to_string();
    let text = Paragraph::new(Line::from(vec![
        Span::styled("Time: ", Style::default().fg(Color::Gray)),
        Span::raw(now),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Clock"));

    f.render_widget(text, area);
}

pub fn render_shortcuts<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let shortcuts = Paragraph::new(Line::from(vec![
        Span::styled("Ctrl+N", Style::default().fg(Color::Green)),
        Span::raw(" - New Node  "),
        Span::styled("Ctrl+X", Style::default().fg(Color::Green)),
        Span::raw(" - Cut Node  "),
        Span::styled("Ctrl+W", Style::default().fg(Color::Green)),
        Span::raw(" - Close App"),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Shortcuts"));

    f.render_widget(shortcuts, area);
}
