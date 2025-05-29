/// Generic drag helper for movable UI elements.
#[derive(Default, Debug, Clone, Copy)]
pub struct DragState {
    pub active: bool,
    pub last_x: u16,
    pub last_y: u16,
}

impl DragState {
    /// Begin a drag operation from the given coordinates.
    pub fn start(&mut self, x: u16, y: u16) {
        self.active = true;
        self.last_x = x;
        self.last_y = y;
    }

    /// End the current drag operation.
    pub fn stop(&mut self) {
        self.active = false;
    }

    /// Calculate the delta from the previous coordinates and update the stored position.
    pub fn delta(&mut self, x: u16, y: u16) -> (i16, i16) {
        let dx = x as i16 - self.last_x as i16;
        let dy = y as i16 - self.last_y as i16;
        self.last_x = x;
        self.last_y = y;
        (dx, dy)
    }
}
