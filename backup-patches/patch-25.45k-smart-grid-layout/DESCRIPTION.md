# Patch 25.45k – Smart Grid Layout + Visual Node Placement

## Goals
- Avoid node collisions using screen-aware spawn logic
- Replace fallback grid logic with scan-based free node placement (top-down)
- Improve spacing using better cell multiples (e.g. *4 instead of *2)
- Auto-Arrange ON = full layout; OFF = preserve manual layout
- Implement void-rs-style `create_anchor` logic

## Files
- src/gemx/interaction.rs
- src/state/mod.rs
- src/layout.rs

## Constants
- SIBLING_SPACING_X = 3
- CHILD_SPACING_Y = 1
- FREE_GRID_COLUMNS = 4

