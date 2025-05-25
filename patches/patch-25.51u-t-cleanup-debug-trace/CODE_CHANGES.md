## Code Changes

### 1. Remove Insert Tracing

In `add_sibling()` and `add_child()`:
```rust
// Remove or comment:
eprintln!("[INSERT] Node..."); 
2. Remove [RENDER TREE] Logs
In render_gemx():

// Remove block logging all nodes
eprintln!("[RENDER TREE]");
3. Strip [F] Prefix on Label Cleanup (Optional)
When promoting fallback:

if node.label.starts_with("[F]") {
    node.label = node.label.trim_start_matches("[F] ").to_string();
}
Or reset all at startup:

for node in state.nodes.values_mut() {
    if node.label.starts_with("[F]") {
        node.label = node.label.replacen("[F] ", "", 1);
    }
}
4. Replace With One-Line Summary (Optional)
if state.debug_input_mode {
    eprintln!("Rendered {} nodes this frame.", drawn_at.len());
}
