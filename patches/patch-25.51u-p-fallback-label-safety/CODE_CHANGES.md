## Code Changes

### 1. Prevent Fallback Label Growth

In fallback section after promotion:
```rust
if let Some(n) = state.nodes.get_mut(&id) {
    if !n.label.starts_with("[F]") {
        n.label = format!("[F] {}", n.label);
    }

    if n.label.len() > 32 {
        n.label.truncate(32);
    }
}
2. Inject Into Drawn At
drawn_at.insert(id, Coords { x: 5, y: GEMX_HEADER_HEIGHT + 2 });
node_roles.insert(id, LayoutRole::Root);
Even if layout_nodes() failed or didn’t process it.

3. Skip Fallback if Already Injected
Guard the fallback loop:

if fallback_promoted_this_session.contains(id)
    || node.label.starts_with("[F]")
    || drawn_at.contains_key(&id)
{
    continue;
}
4. Bonus: Log Once
if state.debug_input_mode {
    eprintln!("⚠ Promoted Node {} to root (label-safe)", id);
}
