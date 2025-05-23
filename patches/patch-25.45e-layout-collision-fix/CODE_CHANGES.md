## 1. Fix Sibling Spacing
- Use balanced offset:
  let mid = siblings.len() / 2;
  let offset = (index - mid) * SIBLING_SPACING_X;
  child.x = parent.x + offset

## 2. Fix Child Spacing
- Always place children below parent:
  child.y = parent.y + CHILD_SPACING_Y

## 3. Prevent Duplicate Positions
- Do not assign same x/y to multiple siblings
- Enforce grid uniqueness for fallback layout

## 4. Enforce Spacing Constants
- SIBLING_SPACING_X = 3
- CHILD_SPACING_Y = 2

