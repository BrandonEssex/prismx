use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;
use crate::layout::RESERVED_ZONE_W;
use crate::render::module_icon::{module_icon, module_label};

pub fn render_zoom_overlay<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    zoom: f32,
    mode: &str,
) {
    let text = format!("Zoom: {:.1}x", zoom);
    let text_width = text.len() as u16;
    let width = text_width + 2;

    let icon_content = format!("{} {}", module_icon(mode), module_label(mode));
    let icon_width = UnicodeWidthStr::width(icon_content.as_str()) as u16 + 2;
    let offset = RESERVED_ZONE_W as u16 + icon_width + 1;

    let right = area.right().saturating_sub(width + offset);
    let x = if right < area.left() { area.left() } else { right };

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
