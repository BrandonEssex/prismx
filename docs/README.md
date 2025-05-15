# 🌌 PrismX — Terminal IDE for the Future

**Version 3.0.0**  
_Modular. Offline-first. Mindmap-powered. Built for clarity, speed, and engineering design workflows._

---

## 📸 Screenshot

+-----------------------------+
| [Mindmap] [Shortcuts] |
| |
| Root Node → Idea → Sub |
| |
+-----------------------------+


---

## 🧠 What is PrismX?

PrismX is a **terminal-based project IDE** where **everything is a tree**.  
It blends mindmaps, dashboards, and plugin-powered tools into one offline-first workspace.

It's built with:
- ⚙️ **Ratatui** for terminal UI rendering
- 🔌 **Dynamic plugins** (like GemDrop) for AI tools, export helpers, and more
- 🧭 A **Zen Mode** for focused workflows
- 🧠 **MindTrace**, a built-in idea linking engine

---

## ⚙️ Features at a Glance

| Feature         | Description |
|----------------|-------------|
| 🧠 Mindmap UI   | Organize thoughts hierarchically with instant keyboard access |
| 🧩 Plugin System| Modular feature expansion (GemDrop, AutoTag, etc.) |
| 📦 Export Engine| Export your mindmaps, plugin data, or project state to JSON |
| 🎛 Dashboard    | Clock + Keybinding overlay + Plugin slots |
| ✍️ Zen Mode     | Distraction-free writing and node creation |
| 🔒 Offline-First| Fully usable with zero network access |

---

## 📂 File Structure

prismx/
├── src/ # Core logic modules
├── config/ # App settings, plugins, launch data
├── plugin/ # Plugin manifests and code
├── tests/ # Unit + feature tests
├── examples/ # Sample plugin & app usage
├── scripts/ # Build & release tooling
├── docs/ # Documentation, QA, changelogs


---

## 🔧 Getting Started

### 🚀 Run from Source

```bash
cargo run --release
🧪 Run Tests
cargo test
To test a specific module:

cargo test --test mindmap_tests
🧩 Plugin System: How It Works

🧬 Plugin Lifecycle
Manifest is detected in config/plugin.json
Entry point path (TOML) is read and parsed
Plugin is given:
Access to internal node tree (MindTrace)
Sandbox I/O permissions
Optional hooks to UI events or keyboard bindings
🔍 Plugin Manifest Example
[plugin]
name = "GemDrop"
version = "0.3.1"
entry = "plugin/GemDrop/gemdrop.rs"
description = "Auto-tagging assistant for nodes"

[features]
auto_tag = true
clipboard_hooks = true
📂 config/plugin.json
{
  "plugins": [
    {
      "name": "GemDrop",
      "path": "plugin/GemDrop/gemdrop.toml",
      "enabled": true
    }
  ]
}
🔁 Architecture Overview

🧠 Runtime Flow Diagram
  ┌────────────┐
  │ User Input │
  └────┬───────┘
       │
       ▼
┌───────────────────┐
│ Input Router      │
│ (crossterm)       │
└────┬──────────────┘
     │
     ▼
┌─────────────┐      ┌──────────────┐
│ UI Renderer │◄────►│ Dashboard    │
│ (Ratatui)   │      │ Widgets      │
└────┬────────┘      └────┬─────────┘
     │                    │
     ▼                    ▼
┌────────────┐      ┌─────────────┐
│ MindTrace  │◄────►│ Mindmap     │
│ (Node Tree)│      │ View Logic  │
└────────────┘      └─────────────┘
🔌 Plugin Injection Flow
┌─────────────┐
│ plugin.json │
└────┬────────┘
     ▼
┌─────────────────────┐
│ Manifest Loader     │
│ (from plugin.rs)    │
└────┬────────────────┘
     ▼
┌────────────────────────┐
│ Inject Plugin Hooks    │
│ - Clipboard, Nodes     │
│ - UI Slots, Export     │
└────────────────────────┘
     ▼
┌─────────────┐
│ Plugin Code │ (Executes within permissions)
└─────────────┘
🎮 Keybindings

Command	Shortcut
Open Settings	Ctrl + .
Open Spotlight	Alt + Space
Create Node	Ctrl + N
Cut Node	Ctrl + X
Quit App	q
Configurable via config/settings.toml.

🧠 MindTrace: Linking Ideas

MindTrace is PrismX’s native node graph engine.

let mut trace = MindTrace::new();
trace.add_node("root", "Main Idea");
trace.link_nodes("root", "child");
Each node can contain:

id
content
children[]
Stored in memory, but exportable.

📦 JSON Export Example

Export all nodes + plugin state:

use prismx::export_engine::export_to_json;
export_to_json(&trace, "output.json").unwrap();
🎨 Theming + Custom Config

[ui]
theme = "dark"
show_clock = true
enable_shortcuts = true

[keybindings]
open_settings = "Ctrl+."
quit = "q"
You can:

Disable the clock
Change spotlight key
Set light/dark mode
🧪 Writing Your First Test

Create a file in tests/:

#[test]
fn test_node_addition() {
    let mut trace = MindTrace::new();
    trace.add_node("a", "Node A");
    assert!(trace.nodes.contains_key("a"));
}
Run with:

cargo test
🧭 Common Pitfalls (Read Me First)

Problem	Fix
UI not rendering?	Ensure your terminal supports ANSI
Shortcut keys not working	Don't type in input box — test in dashboard
Plugin not loading	Check manifest path + plugin.json
Nothing happens on export	Make sure MindTrace is populated
“q” doesn't quit?	Only works in top-level view
📅 Roadmap (Preview)

Version	Highlights
3.1.0	Settings GUI, Plugin Store
4.0.0	Embedded scripting, Node AI agents
5.0.0	Distributed sync + multi-device mindmaps
🤝 Contributing

Fork the repo.
Clone it.
Enable a plugin.
Write a new one.
Submit a PR. 🎉

🔐 License

MIT. No restrictions. Use it. Fork it. Extend it.

🙏 Credits

Built by [Brandon Essex]
Powered by: Rust 🦀, Ratatui, Crossterm