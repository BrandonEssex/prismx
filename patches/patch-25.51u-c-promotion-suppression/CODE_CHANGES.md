## Code Changes

- In `AppState::ensure_valid_roots()`:
```rust
self.root_nodes.retain(|id| self.nodes.contains_key(id));
if self.root_nodes.is_empty() && !self.nodes.is_empty() {
    if let Some((&first_id, _)) = self.nodes.iter().next() {
        if !self.root_nodes.contains(&first_id) {
            self.root_nodes.push(first_id);
            eprintln!("⚠ root_nodes was empty — promoted Node {} to root", first_id);
        }
    }
}
self.root_nodes.sort_unstable();
self.root_nodes.dedup();
In render_gemx() unreachable node handler:
for (id, _) in &state.nodes {
    if !reachable_ids.contains(id) && !state.root_nodes.contains(id) {
        state.root_nodes.push(*id);
        if state.debug_input_mode {
            eprintln!("⚠ Node {} is unreachable — promoting to root", id);
        }
    }
}
Ensure state.root_nodes remains deduplicated:
state.root_nodes.sort_unstable();
state.root_nodes.dedup();
