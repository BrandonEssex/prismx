use ratatui::style::Color;
use crate::beam_color::BeamColor;

/// Color helpers for BeamX styled lines.
pub fn parent_line_color(theme: BeamColor) -> Color {
    let (border, _, _) = theme.palette();
    border
}

pub fn sibling_line_color(theme: BeamColor) -> Color {
    let (_, status, _) = theme.palette();
    status
}
