// PATCHED: src/ui/command_bar.rs — Spotlight-style centered command box

use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph, Clear};
use ratatui::text::{Line, Span};
use ratatui::style::{Style, Modifier, Color};
use ratatui::Frame;

pub fn render_command_bar(frame: &mut Frame<'_>, area: Rect, buffer: &str) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("⌘ Command")
        .border_style(Style::default().fg(Color::Blue));

    let line = Line::from(Span::styled(
        format!("> {}", buffer),
        Style::default()
            .fg(Color::White)
            .bg(Color::Black)
            .add_modifier(Modifier::BOLD),
    ));

    let paragraph = Paragraph::new(vec![line])
        .block(block)
        .style(Style::default().bg(Color::Black));

    frame.render_widget(Clear, area); // Clear background
    frame.render_widget(paragraph, area);
}