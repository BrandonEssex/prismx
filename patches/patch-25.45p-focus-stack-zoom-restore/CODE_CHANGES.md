## Code Changes

- Add `focus_stack: Vec<(NodeID, NodeID, i16, i16)>` to AppState
- Modify Ctrl+W (drill down): push current root, selected, scroll_x/y
- Modify Ctrl+Q (pop up): pop from stack and restore root + viewport
- Add test to verify viewport does not jump when toggling zoom in/out
