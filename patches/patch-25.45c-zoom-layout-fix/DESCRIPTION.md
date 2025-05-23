# Patch 25.45c – Zoom Layout Fix + Collision Protection

## Goals
- Fix tangled and overlapping node visuals caused by zoom
- Normalize zoom scaling across node spacing, label width, and layout margin
- Prevent node collision as zoom scale increases
- Maintain readable layout at all zoom levels

## Core Changes
- Render node positions using: x = layout_x * BASE_SPACING_X * zoom_scale
- Scale horizontal spacing, vertical spacing, and label width
- Add minimum node gap logic to avoid visual overlap
- Treat node positions as logical units (not raw pixels)

## Constants
- BASE_SPACING_X = 20
- BASE_SPACING_Y = 5
- MIN_NODE_GAP = 3

This patch makes zoom render smooth, with Tetris-like spacing instead of collisions.

