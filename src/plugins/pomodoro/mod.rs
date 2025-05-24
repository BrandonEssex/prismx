use super::PluginRender;
use ratatui::{backend::Backend, layout::Rect, widgets::Paragraph, Frame};
use std::time::SystemTime;

#[derive(Copy, Clone)]
pub enum PomodoroState {
    Idle,
    Focus,
    Break,
}

pub struct PomodoroPlugin {
    pub state: PomodoroState,
    pub start_time: Option<SystemTime>,
}

impl PluginRender for PomodoroPlugin {
    fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let label = match self.state {
            PomodoroState::Focus => "Focus Session",
            PomodoroState::Break => "Break Time",
            PomodoroState::Idle => "Pomodoro Idle",
        };
        let para = Paragraph::new(label);
        f.render_widget(para, area);
    }
}
