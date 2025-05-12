// PATCHED: src/prism_icon.rs

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_prism_icon(frame: &mut Frame<'_>, area: Rect, context: &str) {
    let symbol = match context {
        "Zen" => "⨉ Zen",
        "Log" => "⨉ Log",
        "Mindmap" => "⨉ Map",
        "Export" => "⨉ Out",
        _ => "⨉ PrismX",
    };

    let styled = Span::styled(
        symbol,
        Style::default()
            .fg(Color::White)
            .bg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let paragraph = Paragraph::new(Line::from(styled))
        .block(Block::default().borders(Borders::ALL).title("●"))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(paragraph, area);
}