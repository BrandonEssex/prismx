## Code Changes

- New helper: `self.ensure_valid_roots()` in AppState:
```rust
if self.root_nodes.is_empty() && !self.nodes.is_empty() {
    if let Some((&first_id, _)) = self.nodes.iter().next() {
        self.root_nodes.push(first_id);
    }
}
In render_gemx() or at start of frame:
state.ensure_valid_roots();
In recalculate_roles():
Track visited nodes
Log and optionally promote unreachable nodes
for (id, node) in &self.nodes {
    if !visited.contains(id) {
        eprintln!("⚠ Node {} is unreachable — promoting to root", id);
        self.root_nodes.push(*id);
    }
}
Optional: in debug_input_mode, prefix these nodes with "❓" or show grey style
