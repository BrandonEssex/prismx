use ratatui::{
    backend::Backend,
    layout::Rect,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::beam_color::BeamColor;
use super::fonts::FontStyle;

pub fn preview_line(font: FontStyle, color: BeamColor) -> Line<'static> {
    let (fg, _, _) = color.palette();
    Line::from(Span::styled(
        "Sample: The quick brown fox jumps over the lazy dog",
        font.style().fg(fg),
    ))
}

pub fn render_preview<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    font: FontStyle,
    color: BeamColor,
) {
    let line = preview_line(font, color);
    let para = Paragraph::new(line);
    f.render_widget(para, area);
}
