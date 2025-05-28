pub use ratatui::layout::Rect;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Area {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Area {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height }
    }

    pub fn as_rect(self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
}
