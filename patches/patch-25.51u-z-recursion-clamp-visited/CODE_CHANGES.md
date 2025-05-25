## Code Changes

### 1. Clamp Depth Globally
```rust
const MAX_LAYOUT_DEPTH: usize = 50;
2. In layout_recursive_safe() (or layout_nodes equivalent):
Start:

fn layout_recursive_safe(
    nodes: &NodeMap,
    node_id: NodeID,
    ...
    visited: &mut HashSet<NodeID>,
    depth: usize
) -> (i16, i16, i16) {
    if !visited.insert(node_id) || depth > MAX_LAYOUT_DEPTH {
        if debug_mode {
            eprintln!("⚠️ Recursion aborted: Node {} (depth {})", node_id, depth);
        }
        return (y, x, x); // return safe fallback
    }
3. On Exit: Clean Up Visited
(Optional, if layout fails early):

visited.remove(&node_id);
4. Fail-Safe Label (Optional)
if debug_mode {
    let node = nodes.get_mut(&node_id).unwrap();
    node.label = format!("[RECURSE] {}", node.label);
}
