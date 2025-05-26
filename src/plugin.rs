use crate::plugins::{PluginFrame, PluginRender, PomodoroPlugin, CountdownPlugin};
use ratatui::layout::Rect;
use std::collections::VecDeque;
use std::time::{Duration, SystemTime};

pub mod panel;
pub mod registry;

pub struct PluginHost {
    pub active: VecDeque<Box<dyn PluginRender>>,
}

impl PluginHost {
    pub fn new() -> Self {
        Self { active: VecDeque::new() }
    }

    pub fn start_pomodoro(&mut self) {
        self.active.push_back(Box::new(PomodoroPlugin::default()));
    }

    pub fn add_countdown(&mut self, label: String, delta_days: u64) {
        let target = SystemTime::now() + Duration::from_secs(86400 * delta_days);
        self.active.push_back(Box::new(CountdownPlugin { label, target }));
    }

    pub fn render_all(&mut self, f: &mut PluginFrame<'_>, area: Rect) {
        let mut y_offset = 0;
        for plugin in &mut self.active {
            plugin.render(f, Rect::new(area.x, area.y + y_offset, area.width, 3));
            y_offset += 3;
        }
    }
}
