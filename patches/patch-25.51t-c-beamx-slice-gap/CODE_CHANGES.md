## Code Changes

### 1. Right Border Gap for BeamX

In `render_full_border()` or frame renderer:

```rust
let right_x = area.x + area.width - 1;
let top_y = area.y;
let bottom_y = area.y + area.height - 1;
let skip_y_range = (arrow_top_y + 1)..arrow_bottom_y; // Set these Y values based on where arrows animate

for y in top_y..=bottom_y {
    if !skip_y_range.contains(&y) {
        frame.set_cursor(right_x, y);
        frame.print("│");
    }
}
Replace frame.print("│") with the appropriate call in your drawing API.
2. Arrow Color Standardization (in BeamX::render() or arrow painter):
// Outbound arrows: match border color first
let arrow_style = Style::default().fg(border_color); // e.g., Color::White

// Apply shimmer after initial render segment
if shimmer_phase > start {
    apply_shimmer(arrow_id);
}

// Inbound arrows from northeast can remain rainbow, pulse, etc.
