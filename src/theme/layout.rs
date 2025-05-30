pub fn spacing_scale(zoom: f32) -> (i16, i16) {
    let x = (6.0 * zoom).round().max(4.0) as i16;
    let y = (3.0 * zoom).round().max(2.0) as i16;
    (x, y)
}

/// Standard width for each entry in the module switcher list.
/// This keeps icon boxes visually balanced regardless of label length.
pub const SWITCHER_ITEM_WIDTH: u16 = 20;

/// Standard width for small overlay panels like Spotlight and the module
/// switcher.
pub const OVERLAY_WIDTH: u16 = 60;

/// Determine the width of an overlay panel based on the available area. The
/// width never exceeds [`OVERLAY_WIDTH`].
pub fn overlay_width(area_width: u16) -> u16 {
    area_width.min(OVERLAY_WIDTH)
}

/// Fixed width for the Spotlight overlay in columns.
///
/// This constant is an alias for [`OVERLAY_WIDTH`] and retained for backwards
/// compatibility with existing callers.
pub const SPOTLIGHT_WIDTH: u16 = OVERLAY_WIDTH;

/// Determine the width of Spotlight based on the available area.
/// This simply forwards to [`overlay_width`].
pub fn spotlight_width(area_width: u16) -> u16 {
    overlay_width(area_width)
}

/// Whether node labels should wrap to a second line when exceeding the maximum
/// width instead of being truncated.
pub const NODE_WRAP_LABELS: bool = false;

/// Compute the maximum number of characters allowed for a node label at the
/// given zoom scale. This prevents layout breakage from extremely long labels.
pub fn node_max_width(zoom: f32) -> usize {
    let base = (16.0 * zoom).round() as i32;
    base.clamp(8, 32) as usize
}

