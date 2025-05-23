## 1. Rewrite layout_nodes() logic

- Use this exact logic:

let sibling_count = parent.children.len();
let mid = sibling_count / 2;

for (i, &child_id) in parent.children.iter().enumerate() {
    let offset_x = (i as i16 - mid as i16) * SIBLING_SPACING_X;
    child.x = parent.x + offset_x;
    child.y = parent.y + CHILD_SPACING_Y;
}

## 2. Apply layout only when auto_arrange is true
- Manual positions should remain untouched when auto_arrange is false

## 3. Spacing Constants
- SIBLING_SPACING_X = 3
- CHILD_SPACING_Y = 2

## 4. Render nodes scaled by zoom, but layout positions remain unscaled

