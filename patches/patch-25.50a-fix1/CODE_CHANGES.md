// beamx.rs
// - Ensure render_beamx runs AFTER all content
// - Offset drawing x=area.right() - 6, y=area.top()
// - Optionally encapsulate in draw_beamx_right_corner()

// render_settings.rs
// - Confirm left margin/padding so text doesn’t overwrite BeamX
// - Restore title rendering from col 0 if it was shifted
