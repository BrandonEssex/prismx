use ratatui::{backend::Backend, style::Style, widgets::Paragraph, Frame};
use crate::ui::layout::Rect;
use crate::theme::characters::{TOP_LEFT, TOP_RIGHT, BOTTOM_LEFT, BOTTOM_RIGHT, HORIZONTAL, VERTICAL};

/// Render a rounded border around the given area using theme characters.
pub fn draw_rounded_border<B: Backend>(f: &mut Frame<B>, area: Rect, style: Style) {
    let right = area.right().saturating_sub(1);
    let bottom = area.bottom().saturating_sub(1);

    f.render_widget(Paragraph::new(TOP_LEFT).style(style), Rect::new(area.x, area.y, 1, 1));
    f.render_widget(Paragraph::new(TOP_RIGHT).style(style), Rect::new(right, area.y, 1, 1));
    f.render_widget(Paragraph::new(BOTTOM_LEFT).style(style), Rect::new(area.x, bottom, 1, 1));
    f.render_widget(Paragraph::new(BOTTOM_RIGHT).style(style), Rect::new(right, bottom, 1, 1));

    for x in area.x + 1..right {
        f.render_widget(Paragraph::new(HORIZONTAL).style(style), Rect::new(x, area.y, 1, 1));
        f.render_widget(Paragraph::new(HORIZONTAL).style(style), Rect::new(x, bottom, 1, 1));
    }
    for y in area.y + 1..bottom {
        f.render_widget(Paragraph::new(VERTICAL).style(style), Rect::new(area.x, y, 1, 1));
        f.render_widget(Paragraph::new(VERTICAL).style(style), Rect::new(right, y, 1, 1));
    }
}
