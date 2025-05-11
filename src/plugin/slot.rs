use ratatui::{Frame, layout::Rect};

pub type PluginSlotDrawFn = fn(&mut Frame<'_>, Rect);

pub struct PluginSlot {
    pub name: String,
    pub draw: PluginSlotDrawFn,
}

impl PluginSlot {
    pub fn new(name: &str, draw_fn: PluginSlotDrawFn) -> Self {
        Self {
            name: name.to_string(),
            draw: draw_fn,
        }
    }

    pub fn invoke(&self, frame: &mut Frame<'_>, area: Rect) {
        (self.draw)(frame, area);
    }
}