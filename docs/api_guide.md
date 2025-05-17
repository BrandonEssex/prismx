# PrismX Plugin API Reference – v10.1.0+Final

_Modular Plugin Framework • Trust-Scoped • Sandbox-Isolated • WASM-Capable_

---

## 📦 Overview

PrismX supports modular, sandboxed plugins defined via a simple manifest (`plugin.json`). Each plugin can:

- Request **capabilities**
- Register **keyboard shortcuts**
- Define **renderable UI slots**
- Access shard/tag/node data (read/write with permission)
- Hook into the **Signal Bus**
- Store local plugin state
- Declare a **trust score threshold**

---

## 📄 Plugin Manifest Format

All plugins must include a valid JSON manifest (`plugin/plugin-name/manifest.json` or central `config/plugin.json` entry):

```json
{
  "name": "gemdrop",
  "version": "1.1.0",
  "entry": "plugin/gemdrop.wasm",
  "permissions": [
    "shard.read",
    "node.write",
    "ui.slot.render"
  ],
  "keybinds": ["ctrl+g"],
  "slot": "bottom-right",
  "trust": {
    "score": 0.9,
    "strict": true
  }
}
🔐 Capability Declarations

Capability	Description
shard.read	Read-only access to shards
shard.write	Modify or overwrite shard data
node.read	Read node trees
node.write	Add, edit, or delete nodes
tag.modify	Add/remove tags from nodes
ui.slot.render	Draw into a TUI slot
keybind.register	Add new keybinding(s)
signal.subscribe	Listen to runtime events
trust.bypass	Override trust score (restricted)
aether.invoke	Call Aether AI agent API
Capabilities are sandbox enforced and logged in the Trust Score system.
🧭 Slot Rendering

Plugins may draw widgets or visualizations into predefined TUI layout slots:

"slot": "bottom-right"
Available Slots (v10.1.0)
top-left
top-right
bottom-left
bottom-right
center-overlay
zen-overlay
Slots can be themed via theme.toml and dynamically updated via plugin signals.

⌨ Keybinding Declaration

Plugins may declare global or modal keybinds:

"keybinds": ["ctrl+g", "ctrl+shift+x"]
These keys must be namespaced in keymap.rs and are intercepted by the runtime.

Context-sensitive keybindings (e.g., edit mode safe) are automatically respected.
🛰 Signal Bus Integration

Plugins can subscribe to internal runtime signals:

"permissions": ["signal.subscribe"]
Available signals:

Signal	Description
on_node_create	Fired after node is created
on_shard_load	Triggered when a shard is loaded
on_focus_change	Zen focus/personal/work swap
on_plugin_toggle	Plugin enable/disable
on_keypress	Global keypress event
Each plugin receives a scoped signal handler environment. No cross-plugin access is allowed.

🔒 Trust Scoring System

Plugins must declare trust policies:

"trust": {
  "score": 0.95,
  "strict": true
}
Trust Enforcement:
Score 1.0 = System-level / built-in
Score 0.9 = High-trust community/verified plugins
Score 0.5–0.89 = Medium trust (limited access)
Score < 0.5 = Read-only, sandboxed
All plugin activity is logged to docs/aether_log.md with timestamp, scope, and override decision (if any).
💾 Plugin State Persistence

Plugins may use local scoped storage:

plugin/gemdrop/state.json
Accessible only by the plugin runtime; subject to versioned schema migration.

🔍 Plugin Registry

Declared in:

config/plugin.json
Example:

{
  "gemdrop": {
    "path": "plugin/gemdrop.wasm",
    "enabled": true
  },
  "reminder": {
    "path": "plugin/reminder_engine.wasm",
    "enabled": false
  }
}
🧪 Debugging & Trust Logs

All plugin actions, errors, and signals are:

Traced via trust.log
Linked to the AI Override Trail
Available for review at any time
Enable dev debug mode:

prismx --debug-plugin gemdrop
📦 Sample Plugin Templates

See:

plugin_templates/gemdrop
plugin_templates/graph_overlay
plugin_templates/aether_listener
🔧 Development Tools

plugin-dev.sh – Build/test loop
schema.plugin.json – Validation schema for manifest files
plugin-sandbox.rs – TUI signal harness
📚 Related Docs

Developer Guide
End User Manual
Trust Log & Override Trail
Maintained by #ORACLE
Last Updated: v10.1.0+Final