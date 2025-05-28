# Patch 25.46 – Link Editing (delete, swap)

## Goals
- Add ability to remove a link between nodes
- Add ability to swap/reconnect a node to a different parent
- Display confirmation or visual feedback during link edits

## Hotkeys
- Ctrl+Backspace: delete selected link
- Ctrl+Shift+L: rewire link to new parent

## Affected Files
- `src/gemx/graph.rs` or `link.rs`: link management logic
- `src/input/hotkeys.rs`: new shortcuts
- `src/gemx/render.rs`: redraw when links are removed
