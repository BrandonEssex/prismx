use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use std::cmp::max;

pub fn render_zoom_overlay<B: Backend>(f: &mut Frame<B>, area: Rect, zoom: f32) {
    let text = format!("Zoom: {:.1}x", zoom);
    let text_width = text.len() as u16;
    let width = text_width + 2;
    let x = max(
        area.right().saturating_sub(width + 8),
        area.left(),
    );
    let y = area.bottom().saturating_sub(5);
    let style = Style::default().fg(Color::White).bg(Color::Rgb(40, 40, 40));
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(style);
    f.render_widget(block, Rect::new(x, y, width, 3));
    f.render_widget(
        Paragraph::new(text).style(style),
        Rect::new(x + 1, y + 1, text_width, 1),
    );
}
