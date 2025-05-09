use crate::dashboard_widgets::render_clock_widget;
use ratatui::{Frame, layout::Rect};

pub struct Dashboard {
    pub slots: Vec<WidgetSlot>,
}

#[derive(Debug, Clone)]
pub struct WidgetSlot {
    pub id: String,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub widget_type: String,
    pub enabled: bool,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            slots: vec![
                WidgetSlot {
                    id: "clock".into(),
                    x: 0,
                    y: 0,
                    width: 20,
                    height: 3,
                    widget_type: "clock".into(),
                    enabled: true,
                },
            ],
        }
    }

    pub fn render(&self, f: &mut Frame<'_>, area: Rect) {
        for slot in &self.slots {
            if !slot.enabled {
                continue;
            }

            let rect = Rect::new(slot.x, slot.y, slot.width, slot.height);

            match slot.widget_type.as_str() {
                "clock" => render_clock_widget(f, rect),
                _ => {}
            }
        }
    }
}