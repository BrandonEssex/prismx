# Patch 25.45e – Layout Collision Pass

## Goals
- Fix layout collision when siblings and children are created
- Apply proper horizontal spacing between siblings using SIBLING_SPACING_X
- Apply proper vertical spacing between children using CHILD_SPACING_Y
- Ensure fallback and auto-arranged layouts do not stack nodes at identical x/y

## Key Fixes
- Siblings: offset along x-axis by index relative to center
- Children: offset along y-axis from parent
- Avoid zero-offset reuse when adding multiple nodes

## Constants
- SIBLING_SPACING_X = 3
- CHILD_SPACING_Y = 2

This patch enforces spacing across layout modes and zoom levels

