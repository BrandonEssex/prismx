// src/ui/help_overlay.rs

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::state::SidebarView;
use crate::state::AppState;

pub fn render_help_overlay(frame: &mut Frame<'_>, area: Rect, _current: SidebarView, app_state: &AppState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Keyboard Shortcuts");

    let mut help_lines = vec![
        Line::from(Span::styled("Ctrl+Q: Quit", Style::default())),
        Line::from(Span::styled("Ctrl+H: Toggle Help", Style::default())),
        Line::from(Span::styled("Ctrl+D: Dashboard", Style::default())),
        Line::from(Span::styled("Ctrl+Z: Zen Mode", Style::default())),
        Line::from(Span::styled("Ctrl+L: Log View", Style::default())),
        Line::from(Span::styled("Ctrl+M: Mindmap", Style::default())),
        Line::from(Span::styled("Ctrl+E: Export", Style::default())),
        Line::from(Span::styled("Ctrl+Enter: Toggle Command Bar", Style::default())),
        Line::from(Span::styled("Esc: Close Zen/Sidebar", Style::default())),
    ];

    // Add context-aware help here if needed later
    if app_state.view == crate::state::View::Zen {
        help_lines.push(Line::from(Span::styled("You are in Zen Mode", Style::default().add_modifier(Modifier::ITALIC))));
    }

    let paragraph = Paragraph::new(help_lines)
        .block(block)
        .style(Style::default());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1)].as_ref())
        .split(area);

    frame.render_widget(paragraph, chunks[0]);
}