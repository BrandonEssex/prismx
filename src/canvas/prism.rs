use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Paragraph,
    Frame,
};

/// Render a small PrismX emblem in the top-right corner of the canvas.
/// Rendered only in modules that are not visually sensitive (e.g., not Triage).
pub fn render_prism<B: Backend>(f: &mut Frame<B>, area: Rect) {
    // Manual BeamX fallback – draws 6 arrows around a center star
    let x = area.right().saturating_sub(12);
    let y = area.top();

    let border = Style::default().fg(Color::Magenta);
    let status = Style::default().fg(Color::Cyan);
    let prism = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);

    // Exit arrows
    f.render_widget(Paragraph::new("⬉").style(border), Rect::new(x + 0, y + 0, 1, 1));
    f.render_widget(Paragraph::new("⬉").style(border), Rect::new(x + 3, y + 1, 1, 1));
    f.render_widget(Paragraph::new("⬊").style(border), Rect::new(x + 9, y + 3, 1, 1));
    f.render_widget(Paragraph::new("⬊").style(border), Rect::new(x + 11, y + 4, 1, 1));

    // Entry arrows
    f.render_widget(Paragraph::new("⇙").style(status), Rect::new(x + 11, y + 0, 1, 1));
    f.render_widget(Paragraph::new("⇙").style(status), Rect::new(x + 9, y + 1, 1, 1));

    // Center
    f.render_widget(Paragraph::new("✦").style(prism), Rect::new(x + 6, y + 2, 1, 1));
}
