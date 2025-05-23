# Patch 25.45g – Sibling Layout Fix + Free Node Grid

## Goals
- Fix incorrect vertical stacking of siblings
- Ensure siblings are spaced horizontally around the parent
- Children remain stacked vertically below parent
- Free nodes use a grid layout, not vertical stacking
- Prepare for future scroll/zoom awareness

## Constants
- SIBLING_SPACING_X = 3
- CHILD_SPACING_Y = 2
- FREE_GRID_COLUMNS = 4

