use crate::state::icon_state::{BeamDirection, BeamSegment};

pub fn diff_beam_segments(a: &BeamSegment, b: &BeamSegment) -> Vec<String> {
    let mut changes = vec![];
    if a.color != b.color {
        changes.push(format!("Color changed: {:?} -> {:?}", a.color, b.color));
    }
    if a.pulse != b.pulse {
        changes.push(format!("Pulse changed: {} -> {}", a.pulse, b.pulse));
    }
    if a.priority != b.priority {
        changes.push(format!("Priority changed: {} -> {}", a.priority, b.priority));
    }
    changes
}

pub fn summarize_diff(direction: BeamDirection, changes: &[String]) -> String {
    format!(
        "[{:?}] {}",
        direction,
        changes.join(" | ")
    )
}