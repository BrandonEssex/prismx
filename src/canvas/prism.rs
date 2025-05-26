use ratatui::{backend::Backend, layout::Rect, widgets::Paragraph, style::Style, Frame};

/// Render a small PrismX emblem in the top-right corner of the canvas.
pub fn render_prism<B: Backend>(f: &mut Frame<B>, area: Rect) {
    if area.width < 2 || area.height < 1 {
        return;
    }
    let x = area.right().saturating_sub(2);
    let y = area.y;
    let style = Style::default();
    f.render_widget(Paragraph::new("🔷").style(style), Rect::new(x, y, 2, 1));
}
