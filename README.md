# PrismX: Modular Offline Planning System

![version](https://img.shields.io/badge/version-v1.0.0--rc--final-blue)

PrismX is a high-performance, modular, offline-first planning environment built around an intelligent mindmap interface. Designed for engineers, creators, and focused teams, it merges structured planning, dynamic tagging, plugin extensibility, and real-time visual state feedback.

---

## Features

### Core Modules

* **Mindmap-Centric Planning**: Everything is a node. Projects, tasks, ideas, todos.
* **Inbox/Triage View**: Rapid-capture task dumping, with review workflow.
* **Scratchpad / Zen Mode**: Clean interface for writing, outlining, or decompressing.
* **Shortcut-Driven Navigation**: All features controllable via keyboard.

### Intelligent Icon System

* Prism-based icon rendered in TUI corner
* Beam directions reflect subsystem state:

  * Green = Zen Mode
  * Blue = Work Context
  * Red = Urgent Tasks
  * Lavender = Notes
* Animated pulse, sweep, flash based on priority or plugin signals
* Hover and click feedback (if supported by terminal)

### Drop Intelligence

* Drag-and-drop file/URL capture
* Contextual tag + subsystem routing
* Beam reactions on drop
* Plugin intercept hooks (`MindTrace`, `Inbox`, `Vault`, etc.)

### Plugin System

* WASM runtime with sandboxed plugin execution
* Live plugin beam hooks
* Plugin dashboard panel
* Drop schema declaration (accepted MIME, priority)

---

## Installation

### Dependencies

* Rust (>= 1.70.0)
* Unix-like environment (Linux, macOS, WSL)
* Terminal with mouse + color support (Kitty recommended)

### Build

```bash
git clone https://github.com/BrandonEssex/prismx
cd prismx.core
cargo build --release
./target/release/prismx
```

### Optional

```bash
./startup.sh  # cleans, builds, launches with log
```

---

## Usage

### Keyboard Shortcuts

| Key            | Action                  |
| -------------- | ----------------------- |
| `Ctrl+N`       | New node (root/sibling) |
| `Enter`        | New child node          |
| `Tab`          | Enter editing mode      |
| `Ctrl+Q`       | Quit                    |
| `Ctrl+I`       | Toggle PrismX Panel     |
| `Ctrl+Shift+X` | Snapshot icon profile   |
| `Ctrl+Shift+S` | Toggle stealth mode     |

### CmdPalette (`:` prefix)

* `>toggle zen`
* `>bind beam top_left to calendar`
* `>edit prismx icon`
* `>start icon carousel`
* `>rollback icon profile`

---

## Configuration

### `settings.toml` Example:

```toml
[prismx_icon]
enabled = true
position = "top_right"
default_mode_color = "blue"
show_expand_panel = true

[prismx_icon.color_modes]
zen = "green"
work = "blue"
personal = "lavender"
urgent = "red"

[prismx_icon.animation]
pomodoro_active = "pulse:green"
task_due = "flash:orange"

[[prismx_icon.shard_array]]
plugin = "pomodoro"
color = "green"
position = "right"

[[prismx_icon.shard_array]]
plugin = "notes"
color = "lavender"
position = "top"
```

---

## Roadmap (v2.x)

* [x] Icon system w/ animation + hooks
* [x] Plugin visual routing
* [x] Drop intelligence
* [x] Live profile editing & replay
* [ ] TUI beam inspector UI
* [ ] Icon state replay archive + telemetry
* [ ] Remote plugin extension interface

---

## License

MIT License

---

## Maintainer

**Brandon Essex** — [github.com/BrandonEssex](https://github.com/BrandonEssex)

---

*PrismX is designed for terminal power-users. It is fast, keyboard-first, configurable, and offline by default.*
