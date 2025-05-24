## Code Changes

- Added `plugin.rs`:
```rust
use crate::plugins::{PluginRender, PomodoroPlugin, CountdownPlugin};
use std::collections::VecDeque;

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

    pub fn add_countdown(&mut self, label: String, target: SystemTime) {
        self.active.push_back(Box::new(CountdownPlugin { label, target }));
    }

    pub fn render_all<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let mut y_offset = 0;
        for plugin in &mut self.active {
            let rect = Rect::new(area.x, area.y + y_offset, area.width, 3);
            plugin.render(f, rect);
            y_offset += 3;
        }
    }
}
AppState now includes:
pub plugin_host: PluginHost,
In RoutineForge, call:
state.plugin_host.render_all(f, plugin_area);
Add Spotlight parsing example:
if input == "/start pomodoro" {
    state.plugin_host.start_pomodoro();
}
