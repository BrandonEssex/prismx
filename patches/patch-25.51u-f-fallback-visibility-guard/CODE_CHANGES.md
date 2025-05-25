## Code Changes

### 1. Add a fallback tracker to `AppState`
```rust
pub fallback_this_frame: bool,
Reset this at the top of render_gemx():

state.fallback_this_frame = false;
2. In unreachable fallback block inside render_gemx():
Promote only the first unreachable node per frame that has children:

use std::collections::HashSet;
let reachable_ids: HashSet<NodeID> = drawn_at.keys().copied().collect();

for (id, node) in &state.nodes {
    if !reachable_ids.contains(id)
        && !state.root_nodes.contains(id)
        && !state.fallback_this_frame
        && !node.children.is_empty()
    {
        state.root_nodes.push(*id);
        state.root_nodes.sort_unstable();
        state.root_nodes.dedup();
        state.fallback_this_frame = true;

        if state.debug_input_mode {
            eprintln!("⚠ Node {} is unreachable — promoting to root", id);
        }
        break; // Only promote one per frame
    }
}
Nodes with no children are skipped — they won’t fix layout anyway.
3. Optional: Add debug_input_mode toggle to visualize fallback activity
