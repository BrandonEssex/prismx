pub const BASE_SPACING_X: i16 = 20;
pub const BASE_SPACING_Y: i16 = 5;
pub const BASE_CHILD_SPACING_Y: i16 = 3;

#[derive(Clone, Copy, Debug)]
pub enum SpacingProfile {
    Compact,
    Relaxed,
    Tree,
}

pub const DEFAULT_SPACING_PROFILE: SpacingProfile = SpacingProfile::Tree;

pub struct Spacing {
    pub x: i16,
    pub y: i16,
    pub child_y: i16,
}

pub fn spacing_for(profile: SpacingProfile) -> Spacing {
    match profile {
        SpacingProfile::Compact => Spacing { x: 12, y: 3, child_y: BASE_CHILD_SPACING_Y },
        SpacingProfile::Relaxed => Spacing { x: 30, y: 10, child_y: BASE_CHILD_SPACING_Y },
        SpacingProfile::Tree => Spacing { x: 20, y: 5, child_y: BASE_CHILD_SPACING_Y },
    }
}
