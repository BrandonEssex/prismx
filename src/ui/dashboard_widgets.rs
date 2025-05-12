// src/ui/dashboard_widgets.rs

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_dashboard_widget(frame: &mut Frame<'_>, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled("Dashboard", Style::default().add_modifier(Modifier::BOLD)));

    let content = vec![
        Line::from(Span::styled("• Welcome to PrismX", Style::default())),
        Line::from(Span::styled("• Plugin Count: 0", Style::default())),
        Line::from(Span::styled("• Active View: Dashboard", Style::default())),
    ];

    let paragraph = Paragraph::new(content).block(block);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1)].as_ref())
        .split(area);

    frame.render_widget(paragraph, layout[0]);
}