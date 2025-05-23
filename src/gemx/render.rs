use ratatui::{prelude::Rect, Frame, backend::Backend};
use crate::state::AppState;
use crate::screen::gemx::render_gemx;

/// Public wrapper so tests can grep for zoom usage
pub fn render<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    // ensure zoom_scale is referenced from this module
    let _ = state.zoom_scale;
    render_gemx(f, area, state);
}
