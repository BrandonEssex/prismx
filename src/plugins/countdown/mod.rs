use super::PluginRender;
use ratatui::{backend::Backend, layout::Rect, widgets::Paragraph, Frame};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct CountdownPlugin {
    pub label: String,
    pub target: SystemTime,
}

impl PluginRender for CountdownPlugin {
    fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let target = self.target.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let remaining = target.saturating_sub(now);
        let days = remaining / 86400;
        let para = Paragraph::new(format!("D-{}: {}", days, self.label));
        f.render_widget(para, area);
    }
}
