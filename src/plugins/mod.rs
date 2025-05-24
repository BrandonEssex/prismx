use ratatui::{backend::Backend, layout::Rect, Frame};

pub trait PluginRender {
    fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect);
}

pub mod countdown;
pub mod pomodoro;

pub use countdown::CountdownPlugin;
pub use pomodoro::PomodoroPlugin;
