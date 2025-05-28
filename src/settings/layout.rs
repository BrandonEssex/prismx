use ratatui::{layout::Rect, text::Line};
use unicode_width::UnicodeWidthStr;

/// Calculate centered rectangle for the settings panel based on content lines.
pub fn settings_area(area: Rect, lines: &[Line]) -> Rect {
    let content_width = lines
        .iter()
        .map(|l| l.width() as u16)
        .max()
        .unwrap_or(0)
        .saturating_add(4);

    let width = content_width.min(area.width);
    let height = lines.len() as u16 + 2;
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}
