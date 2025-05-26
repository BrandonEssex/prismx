use std::time::{SystemTime, UNIX_EPOCH};
use ratatui::{backend::Backend, layout::Rect, Frame};
use crate::ui::render::draw_beam;
use crate::ui::beamx::{BeamXStyle, BeamXMode};
use std::env;

/// Render a small PrismX emblem in the top-right corner of the canvas.
/// Rendered only in modules that are not visually sensitive (e.g., not Triage).
pub fn render_prism<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let tick = if env::var("PRISMX_TEST").is_ok() {
        1
    } else {
        (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            / 300) as u64
    };
    let style = BeamXStyle::from(BeamXMode::Default);
    draw_beam(f, area, tick, &style);
}
