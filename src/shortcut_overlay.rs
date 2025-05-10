// FINAL FULL FILE DELIVERY
// Filename: /src/shortcut_overlay.rs

use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
    text::{Line, Span},
    style::{Style, Color},
    Frame,
};

pub fn render_shortcuts(frame: &mut Frame<'_>, area: Rect, visible: bool) {
    if !visible {
        return;
    }

    let shortcuts = vec![
        Line::from(Span::styled("Ctrl+E → Export Overlay", Style::default().fg(Color::Green))),
        Line::from(Span::styled("Ctrl+Z → Zen Mode", Style::default().fg(Color::Cyan))),
        Line::from(Span::styled("Ctrl+L → Log Viewer", Style::default().fg(Color::Yellow))),
        Line::from(Span::styled("Ctrl+/ → Show Shortcuts", Style::default().fg(Color::Magenta))),
    ];

    let block = Block::default().title("Shortcuts").borders(Borders::ALL);
    let para = Paragraph::new(shortcuts)
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(para, area);
}