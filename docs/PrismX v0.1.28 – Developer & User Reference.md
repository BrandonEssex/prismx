# PrismX v0.1.28 – Developer & User Reference

## Overview

**PrismX** is a modular, terminal-based productivity hub for engineers, developers, and technical teams. It features a powerful plugin system, customizable TUI/CLI interface, and full offline-first support for projects, shards, mindmaps, routines, and more.

PrismX emphasizes:

* **Extensibility**: via secure WASM plugins or scriptable modules
* **Modularity**: every feature (including AI, encryption, UI panels) can be enabled/disabled
* **Offline-first architecture**: usable in secure, air-gapped environments
* **Visual cognition tools**: radial mindmaps, triage boards, routine planners, and dashboards

---

## Core Concepts

### 1. **Shards & Workspaces**

* A **shard** is a high-level container or context (e.g. `work`, `personal`, `infra`)
* **Workspaces** are layouts of views and modules focused on a specific shard or task
* Internally, shards and workspaces are treated as `NodeGroups`

### 2. **Mindmaps**

* Centered around a root node with unlimited children
* Supports radial, tree, and timeline layouts
* Editable nodes with title, tags, and metadata
* Keyboard-driven navigation and editing
* Persistent JSON-backed save/load to `data/mindmap.json`

### 3. **Triage & Inbox (v0.1.29 target)**

* Supports task capture, prioritization, and shard assignment
* Designed for incident response, backlog grooming, and intake

### 4. **Scratchpad**

* Minimal Markdown-compatible note buffer
* Integrated with Zen Mode for focused writing

### 5. **RoutineForge** (planned)

* Plan routines by day, time, or shard
* Integrates with dashboard and timer modules

---

## Current TUI Features

* [x] Mindmap editor with radial layout and editable nodes
* [x] View switching (radial/tree/timeline toggle)
* [x] Keyboard navigation and inline edit mode
* [x] JSON save/load with UUID support
* [x] Basic visual link rendering between nodes
* [x] Context menu overlay (keyboard trigger only)
* [x] Live layout switching (`m`), editing (`e`), and menu (`c`)
* [x] Modular `Screen` architecture routing draw/render actions

---

## Plugin Architecture (WASM)

* Plugins are compiled to `.wasm` and loaded from `/extensions`
* `prismx-plugin.json` declares:

```json
{
  "name": "my-plugin",
  "version": "1.0",
  "capabilities": ["file:read", "render:overlay"],
  "entry": "plugin.wasm"
}
```

### Supported Plugin Capabilities

* `file:read` / `file:write`
* `network:fetch`
* `render:overlay`
* `keybind:register`
* `bus:emit`, `bus:on`

### Plugin System Modules

* `extension_host` – plugin sandbox, permission engine
* `capability.rs` – defines and enforces declared capabilities
* `plugin_loader.rs` – loads and validates plugin manifests

### Planned Hooks (v0.2+)

* `on_start()`, `on_tick()`, `on_exit()` lifecycle hooks
* Shared reactive signal bus: `emit(Signal::TaskAdded)`
* Plugin-renderable widget slots

---

## Developer Integration Points

### 1. Actions (`actions.rs`)

Add user-triggerable commands like:

```rust
PushEditChar(char),
NavigateNext,
ToggleMindmapLayout,
```

Bind to keyboard in `input.rs`, and route in `screen.rs`.

### 2. Renderer Slots (`view_mindmap.rs`, `screen.rs`)

Every module should expose a `render_xxx()` method that accepts:

```rust
Frame, Rect, &State
```

And is called from the centralized `Screen::draw()`.

### 3. Persistent State

Each module can optionally:

* Store JSON to `data/*.json`
* Use Serde for state structs
* Support versioning + migration via `storage/`

---

## How to Extend PrismX

### Create a Custom Module

1. Add a new file in `src/your_feature.rs`
2. Create a state struct, draw method, and input handler
3. Add to `screen.rs` and `actions.rs`

### Build a Plugin

1. Write a Rust crate with `no_std` + `wasm32-unknown-unknown`
2. Export a minimal WASM file + manifest
3. Drop in `extensions/` directory
4. Register keybindings and slots via manifest

### Modify UI or Shortcuts

* Edit `keymaps.json` or `theme.json` (v0.2.7+)
* Use `screen::handle_action()` to inject rendering

---

## Default Shortcuts

| Key       | Action             |
| --------- | ------------------ |
| `e`       | Enter edit mode    |
| `m`       | Toggle layout mode |
| `c`       | Open node menu     |
| `← ↑ ↓ →` | Navigate nodes     |
| `Enter`   | Confirm edit/menu  |
| `Esc`     | Cancel edit/menu   |
| `q`       | Quit app           |

---

## Use Case Examples

* Technical journaling with visual context maps
* Multi-shard triage management for SREs
* Daily/weekly routine control for focused teams
* Plugin-based knowledge base + task sync
* Full offline ops dashboard for air-gapped sites

---

## Roadmap Highlights

* v0.1.29: Inbox & Triage overhaul
* v0.1.34: Smart templates (routines, triage, incident)
* v0.2.0+: Plugin signals, scripting, capability enforcement
* v1.1.x: Plugin registry, biometric unlock, overlay UIs

---

## Contributing & License

PrismX is licensed under MIT and open to contributors.
Coming soon: plugin SDK, scripting API, and extension CLI tools.

> For bugs, ideas, or integrations, please open issues on GitHub.
