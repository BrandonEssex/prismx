/// Apply the given zoom_scale to a coordinate value.
pub fn apply_zoom(value: u16, zoom_scale: f32) -> u16 {
    ((value as f32) * zoom_scale) as u16
}
