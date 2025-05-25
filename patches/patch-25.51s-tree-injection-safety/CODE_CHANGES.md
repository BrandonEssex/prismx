## Code Changes

- In `add_child()`:
  - Validate that parent is part of a reachable tree
  - If `parent` is not a descendant of any `root_nodes`, promote it:
```rust
if !self.root_nodes.contains(&parent_id) {
    self.root_nodes.push(parent_id);
}
In render_gemx():
If drawn_at.is_empty():
f.render_widget(Paragraph::new("⚠ layout_nodes() returned no visible nodes."), Rect::new(area.x + 2, area.y + 2, 40, 1));
Log orphan nodes:
for (id, node) in &state.nodes {
    if !reachable_ids.contains(id) {
        eprintln!("⚠ Node {} is unreachable from root", id);
    }
}
