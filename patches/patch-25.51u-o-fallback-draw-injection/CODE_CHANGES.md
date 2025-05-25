## Code Changes

### 1. In `render_gemx()` fallback loop:
After promoting a node:
```rust
state.root_nodes.push(*id);
state.root_nodes.sort_unstable();
state.root_nodes.dedup();
state.fallback_this_frame = true;
state.fallback_promoted_this_session.insert(*id);

// 🔥 Inject directly into drawn layout:
drawn_at.insert(*id, Coords { x: 5, y: GEMX_HEADER_HEIGHT + 2 });
node_roles.insert(*id, LayoutRole::Root);
2. In recalculate_roles():
Ensure fallback-injected node has a layout role:

for &id in &self.root_nodes {
    self.layout_roles.insert(id, LayoutRole::Root);
}
3. Suppress fallback retry:
Already done but enforce again:

if fallback_promoted_this_session.contains(id) { continue; }
4. Optional: Visual fallback badge (debug only)
if state.debug_input_mode {
    node.label = format!("[F] {}", node.label);
}
