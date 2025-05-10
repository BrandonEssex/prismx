use ratatui::layout::Rect;

#[derive(Debug, Clone, Default)]
pub struct WidgetSlot {
    pub id: String,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub widget_type: String,
    pub enabled: bool,
}

#[derive(Debug, Default)]
pub struct Dashboard {
    pub slots: Vec<WidgetSlot>,
}

impl Dashboard {
    pub fn new() -> Self {
        Dashboard {
            slots: vec![],
        }
    }

    pub fn render(&self, _f: &mut ratatui::Frame<'_>, _area: Rect) {
        // Placeholder for rendering dashboard widgets
    }
}