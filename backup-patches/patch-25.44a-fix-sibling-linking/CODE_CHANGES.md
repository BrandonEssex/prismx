## Code Changes

- Ensure new sibling nodes link as children of current node, not the reverse
- Prevent a node from ever becoming its own parent
- Add test coverage for `handle_enter_key()`, `add_sibling_node()`, `add_child_node()`
- Confirm structure remains valid after every new node creation
