## Code Changes

### 1. Define command suggestion list:
```rust
let suggestions = vec!["/triage", "/zen", "/settings", "/goto", "/plugin"];
2. Match input prefix:
let matches: Vec<_> = suggestions.iter()
    .filter(|s| s.starts_with(spotlight_input.as_str()))
    .collect();
Render below the input panel:

for (i, suggestion) in matches.iter().enumerate() {
    f.render_widget(Paragraph::new(*suggestion), Rect::new(x, y + 2 + i as u16, width, 1));
}
