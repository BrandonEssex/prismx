use super::{PluginFrame, PluginRender};
use ratatui::{layout::Rect, widgets::Paragraph};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct CountdownPlugin {
    pub label: String,
    pub target: SystemTime,
}

impl PluginRender for CountdownPlugin {
    fn render(&mut self, f: &mut PluginFrame<'_>, area: Rect) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let target = self.target.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let remaining = target.saturating_sub(now);
        let days = remaining / 86400;
        let para = Paragraph::new(format!("D-{}: {}", days, self.label));
        f.render_widget(para, area);
    }
}
