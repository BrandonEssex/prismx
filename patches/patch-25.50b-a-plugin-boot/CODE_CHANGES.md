## Code Changes

- Created:
  - `plugins/mod.rs`
  - `plugins/countdown/mod.rs`
  - `plugins/pomodoro/mod.rs`

- Trait:
```rust
pub trait PluginRender {
    fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect);
}
CountdownPlugin:
pub struct CountdownPlugin {
    pub label: String,
    pub target: SystemTime,
}
impl PluginRender for CountdownPlugin { ... }
PomodoroPlugin:
pub struct PomodoroPlugin {
    pub start_time: Option<SystemTime>,
    pub state: PomodoroState,
}
enum PomodoroState { Focus, Break, Idle }
Registered in RoutineForge (temporarily) for visual test hook
Expose:
pub use countdown::CountdownPlugin;
pub use pomodoro::PomodoroPlugin;
pub use PluginRender;
