# Patch 25.45n-hb – Span Clamp & Label-Aware Sibling Positioning

## Goals
- Fix horizontal collisions in tidy layout (25.45n-h)
- Clamp child span to at least label width + MIN_NODE_GAP
- Ensure spacing even when siblings are visually narrow
- Preserve role mapping and min_x shift behavior from Codex patch
