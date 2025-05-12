// PATCHED: src/prism_icon.rs — ASCII-style PrismX icon with context

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Span, Line};
use ratatui::widgets::{Paragraph};
use ratatui::Frame;

pub fn render_prism_icon(frame: &mut Frame<'_>, area: Rect, context: &str) {
    let content = format!("╳ PrismX [{}]", context);

    let styled = Span::styled(
        content,
        Style::default()
            .fg(Color::Cyan)
            .bg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let paragraph = Paragraph::new(Line::from(styled)).style(Style::default().bg(Color::Black));

    frame.render_widget(paragraph, area);
}