## 1. Fix Parent-Child Layout
- Update layout_nodes() so children share a single row below the parent
- CHILD_SPACING_Y = 2
- SIBLING_SPACING_X = 3

## 2. Remove Excessive Horizontal Offset
- Center siblings horizontally around the parent using `(i - mid) * SIBLING_SPACING_X`

## 3. Preserve Zoom
- All spacing must multiply by zoom_scale at render time
- Structure must remain consistent regardless of scale

## 4. Future Preparation
- Allow spacing_profile to define stagger vs stack behavior
- Optional: add layout_hint to Node in the future

