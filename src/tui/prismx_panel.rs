use ratatui::style::Color;
use crate::state::icon_state::{BeamState, BeamDirection, BeamSegment};

pub fn update_prism_beam(state: &mut BeamState, direction: BeamDirection, color: Color, priority: u8) {
    let segment = match direction {
        BeamDirection::TopLeft => &mut state.top_left,
        BeamDirection::TopRight => &mut state.top_right,
        BeamDirection::BottomLeft => &mut state.bottom_left,
        BeamDirection::BottomRight => &mut state.bottom_right,
    };

    if priority >= segment.priority {
        segment.color = color;
        segment.priority = priority;
    }
}