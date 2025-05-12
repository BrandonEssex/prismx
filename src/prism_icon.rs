// PATCHED: src/prism_icon.rs — Context-aware color for PrismX icon

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Span, Line};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

fn view_color(view: &str) -> Color {
    match view {
        "Zen"      => Color::Green,
        "Log"      => Color::Yellow,
        "Mindmap"  => Color::Cyan,
        "Export"   => Color::Blue,
        _          => Color::White,
    }
}

pub fn render_prism_icon(frame: &mut Frame<'_>, area: Rect, context: &str) {
    let color = view_color(context);
    let content = format!("╳ PrismX [{}]", context);

    let styled = Span::styled(
        content,
        Style::default()
            .fg(color)
            .add_modifier(Modifier::BOLD),
    );

    let paragraph = Paragraph::new(Line::from(styled))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(paragraph, area);
}