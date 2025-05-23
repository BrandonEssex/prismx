
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


# Patch 25.49 – Visual Link Cleanup + Child Detach Fix

## Goals
- Improve visual rendering of node link arrows (cleaner layout, arrowheads)
- Add drag/hover indicators for selected or dragged nodes
- Automatically detach child from parent if node is moved far away
- Ensure detached nodes are promoted to top-level (free nodes)

## Visuals
- Replace basic link lines with `→`, `↳`, `╰─`, or `*`
- During drag: highlight node or connection
- Optional: fade inactive nodes or dim siblings

## Behavior
- If a node is dragged far enough from its parent, sever the parent-child link
- Remove node from parent.children and nullify its .parent
