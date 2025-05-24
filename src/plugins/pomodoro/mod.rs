use super::{PluginFrame, PluginRender};
use ratatui::{layout::Rect, widgets::Paragraph};
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

impl Default for PomodoroPlugin {
    fn default() -> Self {
        Self {
            state: PomodoroState::Idle,
            start_time: None,
        }
    }
}

impl PluginRender for PomodoroPlugin {
    fn render(&mut self, f: &mut PluginFrame<'_>, area: Rect) {
        let label = match self.state {
            PomodoroState::Focus => "Focus Session",
            PomodoroState::Break => "Break Time",
            PomodoroState::Idle => "Pomodoro Idle",
        };
        let para = Paragraph::new(label);
        f.render_widget(para, area);
    }
}
