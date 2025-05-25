# Patch 25.51u-o – View-Aware Layout & Cluster Rendering

## Goals
- Make auto-arrange aware of the visible screen size.
- Stack child nodes vertically and wrap to new columns when they no longer fit in
  the current view height.
- Prevent sibling collisions and provide debug output when nodes overflow.

## Affected Modules
- `src/layout/mod.rs`
- `src/screen/gemx.rs`
- `src/gemx/interaction.rs`
- `tests/insertion.rs`
