use super::base::Rect;

/// Return a new Rect reduced by uniform padding.
pub fn inset(rect: Rect, padding: u16) -> Rect {
    let width = rect.width.saturating_sub(padding * 2);
    let height = rect.height.saturating_sub(padding * 2);
    Rect::new(rect.x + padding, rect.y + padding, width, height)
}

/// Center a rectangle of the given size within another area.
pub fn center_rect(area: Rect, width: u16, height: u16) -> Rect {
    let w = width.min(area.width);
    let h = height.min(area.height);
    let x = area.x + (area.width - w) / 2;
    let y = area.y + (area.height - h) / 2;
    Rect::new(x, y, w, h)
}
