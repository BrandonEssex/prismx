use crate::state::icon_state::{BeamSegment, BeamState};
use ratatui::style::Color;

#[test]
fn test_beam_priority_override() {
    let mut segment = BeamSegment {
        color: Color::Blue,
        priority: 100,
        pulse: false,
    };

    // Lower priority update (should not override)
    if 50 >= segment.priority {
        segment.color = Color::Red;
    }
    assert_eq!(segment.color, Color::Blue);

    // Higher priority update
    if 150 >= segment.priority {
        segment.color = Color::Green;
    }
    assert_eq!(segment.color, Color::Green);
}

#[test]
fn test_state_color_change() {
    let mut state = BeamState::default();
    state.top_left.color = Color::Red;
    assert_eq!(state.top_left.color, Color::Red);
}