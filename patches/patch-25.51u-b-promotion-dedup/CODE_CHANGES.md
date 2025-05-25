## Code Changes

- Modify `ensure_valid_roots()`:
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
In render_gemx() unreachable check:
Only promote if not already in root_nodes
Suppress log if already logged this frame (or debounce)
Add optional debug_log_once_per_frame() util:
static mut LOGGED_THIS_FRAME: bool = false;

if !LOGGED_THIS_FRAME {
    eprintln!("⚠ root_nodes was empty — recovery injected.");
    LOGGED_THIS_FRAME = true;
}
(You can reset it at the start of each render_gemx() call.)
