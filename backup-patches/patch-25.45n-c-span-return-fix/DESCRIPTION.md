# Patch 25.45n-c – Span Return & Parent Anchor Fix

## Goals
- Fix parent positioning when centering over children
- Properly return min_x and max_x from layout_recursive_safe
- Assign node.x after all children are laid out
- Prevent visual corruption and collapsed node render

## Dependencies
- Follows up on 25.45n and 25.45n-b
- Target file: src/layout.rs

