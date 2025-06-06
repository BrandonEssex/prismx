use ratatui::{backend::Backend, layout::Rect, Frame};
use crate::beamx::render_full_border;
use crate::state::AppState;
use crate::ui::beamx::{render_beam, BeamXStyle, BeamXMode};
use crate::triage::render_triage_panel as render_panel;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn render_triage<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut AppState) {
    let style = state.beam_style_for_mode("triage");
    render_panel(f, area, state);
    render_full_border(f, area, &style, true, false);
    let tick = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() / 300) as u64;
    let mut bx_style = BeamXStyle::from(BeamXMode::Triage);
    let (b, s, p) = state.beamx_panel_theme.palette();
    bx_style.border_color = b;
    bx_style.status_color = s;
    bx_style.prism_color = p;
    if state.beamx_panel_visible {
        render_beam(f, area, tick, &bx_style);
    }
}
