#[derive(Debug, Clone)]
pub struct ViewState {
    pub zoom_scale: f32,
    pub offset_x: i16,
    pub offset_y: i16,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            zoom_scale: 1.0,
            offset_x: 0,
            offset_y: 0,
        }
    }
}
