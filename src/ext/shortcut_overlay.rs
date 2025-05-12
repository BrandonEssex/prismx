// src/ext/shortcut_overlay.rs

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_shortcut_overlay(frame: &mut Frame<'_>, area: Rect) {
    let block = Block::default()
        .title("Shortcuts")
        .borders(Borders::ALL);

    let items = vec![
        Line::from("Ctrl+D - Dashboard"),
        Line::from("Ctrl+Z - Zen Mode"),
        Line::from("Ctrl+L - Log View"),
        Line::from("Ctrl+M - Mindmap"),
        Line::from("Ctrl+E - Export"),
        Line::from("Esc - Close Overlay"),
    ];

    let paragraph = Paragraph::new(items)
        .block(block)
        .alignment(Alignment::Left)
        .style(Style::default().add_modifier(Modifier::BOLD));

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1)].as_ref())
        .split(area);

    frame.render_widget(paragraph, layout[0]);
}