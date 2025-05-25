# Patch 25.51u-m – Root Role Assignment + Fallback Safety Guard

## Goals
- Assign `LayoutRole::Root` to all promoted root nodes
- Prevent fallback from re-promoting already drawn roots
- Guarantee root nodes are visible even with no children
