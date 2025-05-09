use ratatui::backend::Backend;
use ratatui::Frame;
use crate::screen::Screen;
use crate::state::AppState;

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, screen: &mut Screen, state: &mut AppState) {
    screen.draw(f, state);
}