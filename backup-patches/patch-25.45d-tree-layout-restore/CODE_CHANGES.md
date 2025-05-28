## 1. Fix Parent-Child Layout
- Update layout_nodes() to stack children vertically
- BASE_CHILD_SPACING_Y = 3

## 2. Remove Excessive Horizontal Offset
- Set child.x = parent.x by default
- Optionally stagger siblings by +1 X if needed for spacing

## 3. Preserve Zoom
- All spacing must multiply by zoom_scale at render time
- Structure must remain consistent regardless of scale

## 4. Future Preparation
- Allow spacing_profile to define stagger vs stack behavior
- Optional: add layout_hint to Node in the future

