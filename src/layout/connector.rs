use crate::node::Node;

/// Calculate the vertical offset of the sibling beam for a parent node.
pub fn beam_y(parent: &Node, spacing_y: i16) -> i16 {
    parent.y + (spacing_y - 1).max(1)
}

/// Starting and ending coordinates for the vertical line from parent to its
/// sibling beam.
pub fn parent_line(parent: &Node, spacing_y: i16) -> ((i16, i16), (i16, i16)) {
    let start = (parent.x, parent.y + 1);
    let end = (parent.x, beam_y(parent, spacing_y));
    (start, end)
}

/// Starting and ending coordinates for the vertical connector down to a child.
pub fn child_line(child: &Node, beam_y: i16) -> ((i16, i16), (i16, i16)) {
    let start = (child.x, beam_y);
    let end = (child.x, child.y);
    (start, end)
}
