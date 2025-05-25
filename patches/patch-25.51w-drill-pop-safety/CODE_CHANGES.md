## Code Changes

### 1. Validate Drill Target Before Entering

In `drill_selected()`:
```rust
if let Some(id) = self.selected {
    if self.nodes.contains_key(&id) {
        self.view_stack.push(self.drawing_root.take());
        self.drawing_root = Some(id);
        self.scroll_x = 0;
        self.scroll_y = 0;
    } else {
        eprintln!("❌ Drill failed: node not found.");
    }
}
2. Pop Drill Safely
In pop_stack():

if let Some(prev) = self.view_stack.pop() {
    if let Some(id) = prev {
        if self.nodes.contains_key(&id) {
            self.drawing_root = Some(id);
            return;
        }
    }
}

self.drawing_root = None; // fall back to root_nodes
3. Clamp Zoom and Scroll Reset
On drill/pop:

self.scroll_x = 0;
self.scroll_y = 0;
self.zoom_scale = 1.0;
4. Optional: Highlight Drill Root on Render
In render_gemx():

if let Some(id) = self.drawing_root {
    if let Some(coords) = drawn_at.get(&id) {
        // style selected root, or annotate it
    }
}
