## Code Changes

- Add `PluginSettingsTab` struct with:
  - tab_title
  - render_fn
- Register plugin tabs via plugin manifest or registry
- On Settings screen, draw tabs dynamically: system tabs + plugin tabs
