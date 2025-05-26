use ratatui::{backend::Backend, layout::Rect, Frame};

use crate::state::AppState;
use crate::plugin::panel::render_plugin_panel;

pub fn render_plugin<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    render_plugin_panel(f, area, state);
}
