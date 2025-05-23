pub use crate::layout::{
    layout_nodes,
    ensure_manual_positions,
    Coords,
    CHILD_SPACING_Y,
    SIBLING_SPACING_X,
};

// Layout algorithm summary:
// child.x = parent.x + (index - mid) * SIBLING_SPACING_X
// child.y = parent.y + CHILD_SPACING_Y
