use ratatui::{
    layout::Rect,
    style::{Style, Color},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use chrono::Local;

pub fn render_clock_widget(f: &mut Frame, area: Rect) {
    let now = Local::now().format("%H:%M:%S").to_string();
    let text = Paragraph::new(Line::from(vec![
        Span::styled("Time: ", Style::default().fg(Color::Gray)),
        Span::raw(now),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Clock"));
    f.render_widget(text, area);
}

pub fn render_shortcuts(f: &mut Frame, area: Rect) {
    let shortcuts = Paragraph::new(Line::from(vec![
        Span::styled("Ctrl+N", Style::default().fg(Color::Green)),
        Span::raw(" New  "),
        Span::styled("Ctrl+X", Style::default().fg(Color::Green)),
        Span::raw(" Cut  "),
        Span::styled("Ctrl+V", Style::default().fg(Color::Green)),
        Span::raw(" Paste  "),
        Span::styled("Ctrl+Z", Style::default().fg(Color::Green)),
        Span::raw(" Zen  "),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Shortcuts"));
    f.render_widget(shortcuts, area);
}
