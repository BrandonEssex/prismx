## 1. Sibling Fix

Replace layout logic with:

let count = siblings.len();
let mid = count / 2;

for (i, &child_id) in siblings.iter().enumerate() {
    let offset_x = (i as i16 - mid as i16) * SIBLING_SPACING_X;
    child.x = parent.x + offset_x;
    child.y = parent.y + CHILD_SPACING_Y;
}

## 2. Free Node Grid

If node has no parent, assign position in a grid:

child.x = (i % FREE_GRID_COLUMNS) * SIBLING_SPACING_X;
child.y = (i / FREE_GRID_COLUMNS) * CHILD_SPACING_Y;

## 3. Apply in layout_nodes or assign_positions

Only run when auto_arrange is enabled.

Do not overwrite manually dragged positions.

