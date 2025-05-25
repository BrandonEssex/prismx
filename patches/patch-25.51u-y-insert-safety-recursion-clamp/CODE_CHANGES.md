## Code Changes

### 1. Prevent Circular Parent/Child Relationships

In `add_child()`:
```rust
if parent_id == Some(new_id) {
    eprintln!("❌ Invalid parent/child: node cannot parent itself.");
    return;
}
Also ensure:

if self.nodes.get(&parent_id).map(|n| n.children.contains(&parent_id)).unwrap_or(false) {
    eprintln!("❌ Cycle detected: parent already has itself as child.");
    return;
}
2. Assign Safe Coordinates for Manual Layout
In add_child() if !auto_arrange:

let base_x = 6 + ((self.nodes.len() as i16) % 10) * SIBLING_SPACING_X;
let base_y = GEMX_HEADER_HEIGHT + 2 + ((self.nodes.len() as i16) / 10) * CHILD_SPACING_Y;

node.x = base_x;
node.y = base_y;
3. Clamp Layout Recursion in layout_recursive_safe
if !visited.insert(node_id) || depth > MAX_LAYOUT_DEPTH {
    if state.debug_input_mode {
        eprintln!("⚠️ Layout recursion exited: Node {} already visited or depth exceeded", node_id);
    }
    return (y, x, x);
}
4. Optional: Visual badge on skipped node (dev only)
if debug_input_mode && skipped {
    node.label = format!("[!RECURSE] {}", node.label);
}
