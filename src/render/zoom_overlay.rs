use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

pub fn render_zoom_overlay<B: Backend>(f: &mut Frame<B>, area: Rect, zoom: f32) {
    let text = format!("[ Zoom: {:.1}x ]", zoom);
    let width = text.len() as u16;
    let rect = Rect::new(area.x + 1, area.bottom().saturating_sub(2), width, 1);
    let style = Style::default().fg(Color::White).bg(Color::DarkGray);
    f.render_widget(Paragraph::new(text).style(style), rect);
}
