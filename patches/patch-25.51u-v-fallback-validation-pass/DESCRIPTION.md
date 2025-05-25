# Patch 25.51u-v – Fallback Promotion Validation Pass

## Goals
- Enforce that fallback-promoted nodes are valid and visible
- Ensure promoted nodes have (x, y), LayoutRole::Root, and drawn_at entries
- Log a warning or abort fallback if these are missing
- Stabilize layout integrity after fallback insert
