# Patch 25.49 – Visual Link Cleanup & Drag Detach

## Goals
- Improve readability of tree links in GemX
- Highlight nodes while dragging
- Automatically detach a node if dragged far from its parent

## Details
- Replace ASCII branch glyphs with arrow styled symbols (`↳`, `╰─`)
- During a drag operation the dragged node is rendered with a distinct style
- If the distance between a child and its parent exceeds ~30 units while dragging, the child is detached and becomes a root node

## Affected Files
- `src/screen/gemx.rs`
- `src/gemx/interaction.rs`
