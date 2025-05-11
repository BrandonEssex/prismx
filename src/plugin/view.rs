use ratatui::{layout::Rect, Frame};

use crate::plugin::slot::PluginSlot;

pub struct PluginViewport {
    pub slot: PluginSlot,
    pub area: Rect,
}

impl PluginViewport {
    pub fn render(&self, frame: &mut Frame<'_>) {
        self.slot.invoke(frame, self.area);
    }
}