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

