use crate::theme::layout::spacing_scale;

/// Public wrapper used by renderers to compute character spacing
/// for a given zoom level.
pub fn spacing_for_zoom(zoom: f32) -> (i16, i16) {
    spacing_scale(zoom)
}

