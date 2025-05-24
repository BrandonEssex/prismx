## Code Changes

- Extend `Node` struct: `manual_coords: Option<(i16, i16)>`
- In drag handlers (interaction.rs), set manual_coords on drop if auto_arrange is false
- In layout.rs, skip any node with manual_coords when auto-arrange is ON
- Add toggle logic: when switching to manual mode, seed manual_coords from layout result
