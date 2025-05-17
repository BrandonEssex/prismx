# PrismX Developer Integration Guide – v10.1.0+Final

_Modular, Extensible, Offline-First Engineering Interface_  
_Designed for Plugin Authors, UI Extenders, and AI Signal Integrators_

---

## 📐 Purpose

This guide helps developers:

- Create and register plugins
- Extend the TUI with panels, slots, or widgets
- Declare new keybindings and input handlers
- Use the PrismX Signal Bus for real-time hooks
- Interact safely with the core data model (shards, nodes, tags)
- Integrate with Project Aether (AI override APIs)

---

## 📁 Project Structure

prismx/
├── src/
│ ├── core/ # Signals, traits, state machine
│ ├── shard/ # Node/tree model + JSON database
│ ├── plugin/ # Plugin runtime, sandboxing, WASM host
│ ├── tui/ # Mindmap, dashboard, widgets
│ ├── config/ # User-facing JSON/TOML files
│ └── ai/ # Project Aether agents + audit trail
├── plugin/ # User + system plugins (WASM or static)
├── config/plugin.json # Active plugin registry
└── theme.toml # Theming + font/icon configuration


---

## 🧩 Plugin Development

### 1. Create a Manifest

```json
{
  "name": "reminder_engine",
  "version": "1.0.0",
  "entry": "plugin/reminder_engine.wasm",
  "permissions": ["shard.read", "ui.slot.render"],
  "keybinds": ["ctrl+r"],
  "slot": "top-right",
  "trust": {
    "score": 0.85,
    "strict": true
  }
}
2. Implement Hooks (Rust, TinyGo, AssemblyScript)
Entry point must export on_load() and optionally:
on_signal()
on_key()
on_tick() (for background polling)
render() (for TUI slot output)
3. Register Plugin
In config/plugin.json:

"reminder_engine": {
  "path": "plugin/reminder_engine.wasm",
  "enabled": true
}
🔑 Keybinding System

Declare New Keys
All keybindings must be mapped in:

config/keymap.rs
Keybindings can be scoped by:

Global (ctrl + k)
Mode-based (vim.normal.k)
Context-aware (disabled during text input)
Plugins can declare additional bindings via plugin.json.

🛰 Signal Bus API

Listen to system events via:

fn on_signal(signal: &Signal) {
  match signal.name {
    "on_node_create" => { ... }
    "on_focus_change" => { ... }
    _ => {}
  }
}
Signal types include:

Name  Description
on_node_create  Node added to current shard
on_shard_load New shard loaded into session
on_focus_change Zen mode context switched
on_plugin_toggle  Plugin enabled or disabled
on_keypress Captured global keypress
🎨 UI Slot Rendering

Slots are virtual TUI containers:

"slot": "bottom-right"
Render with:

fn render(ctx: &mut RenderContext) {
  ctx.print("Reminder: Check extinguisher tags.");
}
Slots can be themed and toggled via layout config (dashboard.json).

⚙ Configuration Formats

plugin.json → Plugin registry (required)
theme.toml → Colors, icons, font
dashboard.json → Slot positions, widget layout
keymap.rs → User & plugin keybindings
plugin/<name>/state.json → Local plugin save file
🔐 Trust & Permission Handling

Plugin requests must pass trust checks:

Scored from 0.0 to 1.0
Logged in docs/aether_log.md
Plugins with strict=true are blocked on under-trust
Trusted plugin metadata is cached
Override manually (admin only):

{
  "trust_override": {
    "reminder_engine": true
  }
}
🧠 Project Aether Integration (Optional)

Plugins can access AI modules:

"permissions": ["aether.invoke"]
Available agents:

Agent Description
tagger  Auto-tag suggestions from text
analyzer  Detects incomplete tasks
summarizer  Creates shard summary nodes
recall  Fetches past relevant context
Calls are rate-limited and logged.

🧪 Testing & Debugging

cargo run -- --plugin-debug reminder_engine
All plugin calls are logged to plugin.log
Runtime sandbox errors will appear in stderr
Use prismx --debug to enable full system tracing
📚 Related Files

Plugin API Reference
End User Manual
Trust Log
[Keybinding Cheat Sheets](cheatsheet.md / cheatsheet-vim.md)