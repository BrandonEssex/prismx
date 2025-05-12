// src/ui/help_overlay.rs

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Span, Spans};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use crate::state::SidebarView;

pub fn render_help_overlay(frame: &mut Frame<'_>, area: Rect, current: SidebarView) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Keyboard Shortcuts");

    let help_items = vec![
        Spans::from(vec![Span::raw("q: Quit")]),
        Spans::from(vec![Span::raw("Tab: Cycle Sidebar")]),
        Spans::from(vec![Span::raw("Ctrl+S: Save")]),
        Spans::from(vec![Span::raw("Ctrl+N: New Node")]),
        Spans::from(vec![Span::raw("Esc: Close Sidebar")]),
    ];

    let help = Paragraph::new(help_items)
        .block(block)
        .style(Style::default().add_modifier(Modifier::ITALIC));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1)].as_ref())
        .split(area);

    frame.render_widget(help, chunks[0]);
}