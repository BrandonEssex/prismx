# Patch 25.45f – Void-Style Tree Layout

## Goals
- Align PrismX node layout with void-rs spacing and hierarchy
- Center siblings horizontally with dynamic offset per index
- Stack children vertically beneath parents with consistent vertical spacing
- Prevent overlapping X/Y coordinates at all zoom levels
- Maintain shape and structure regardless of zoom

## Strategy
- Layout logic will center children under the parent
- Sibling nodes spread horizontally using their index relative to the group
- Spacing constants are zoom-agnostic (only applied in render)

## Constants
- SIBLING_SPACING_X = 3
- CHILD_SPACING_Y = 2

