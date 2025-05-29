use ratatui::{backend::Backend, layout::Rect, Frame};
use crate::state::AppState;
use crate::ui::status::render_status;

pub fn render_status_bar<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    render_status(f, area, state);
}
