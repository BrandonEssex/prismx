pub fn spacing_scale(zoom: f32) -> (i16, i16) {
    let x = (6.0 * zoom).round().max(4.0) as i16;
    let y = (3.0 * zoom).round().max(2.0) as i16;
    (x, y)
}

/// Fixed width for the Spotlight overlay in columns.
pub const SPOTLIGHT_WIDTH: u16 = 60;

/// Determine the width of Spotlight based on the available area.
///
/// The width never exceeds [`SPOTLIGHT_WIDTH`] and is independent of user input
/// length so the panel remains stable as commands are typed.
pub fn spotlight_width(area_width: u16) -> u16 {
    area_width.min(SPOTLIGHT_WIDTH)
}

