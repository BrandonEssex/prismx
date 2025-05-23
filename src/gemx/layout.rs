use crate::node::{NodeID, NodeMap};

pub const SIBLING_SPACING_X: i16 = 3;
pub const CHILD_SPACING_Y: i16 = 2;

#[allow(dead_code)]
pub fn apply_layout(nodes: &mut NodeMap, parent_id: NodeID) {
    if let Some(parent) = nodes.get(&parent_id).cloned() {
        let sibling_count = parent.children.len();
        let mid = sibling_count / 2;
        for (i, &child_id) in parent.children.iter().enumerate() {
            if let Some(child) = nodes.get_mut(&child_id) {
                let offset_x = (i as i16 - mid as i16) * SIBLING_SPACING_X;
                child.x = parent.x + offset_x;
                child.y = parent.y + CHILD_SPACING_Y;
            }
        }
    }
}

