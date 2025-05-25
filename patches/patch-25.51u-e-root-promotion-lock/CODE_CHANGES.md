## Code Changes

### 1. Add to `AppState`:

```rust
pub last_promoted_root: Option<NodeID>,
Initialize this to None in your AppState::new() constructor.

2. Update ensure_valid_roots() logic:
self.root_nodes.retain(|id| self.nodes.contains_key(id));

if self.root_nodes.is_empty() && !self.nodes.is_empty() {
    if let Some((&first_id, _)) = self.nodes.iter().next() {
        if Some(first_id) != self.last_promoted_root {
            self.root_nodes.push(first_id);
            self.last_promoted_root = Some(first_id);
            eprintln!("⚠ root_nodes was empty — promoted Node {} to root", first_id);
        }
    }
}
self.root_nodes.sort_unstable();
self.root_nodes.dedup();
3. Optional: Reset last_promoted_root when a valid selection is made
