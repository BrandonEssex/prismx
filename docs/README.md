# 🌌 PrismX — Modular TUI IDE with Mindmaps, Zen Mode, and Plugin Power

**Version:** 3.0.0  
**Language:** Rust 🦀  
**License:** MIT  
**Status:** ✅ Stable, Fully Functional

---

## 📦 What is PrismX?

PrismX is a terminal-first, modular, offline-capable TUI system designed for:
- Hierarchical planning via mindmaps
- Focused Zen-mode writing
- Extendable plugin architecture
- Fast keyboard-driven workflows

---

## 🚀 Features

| Feature            | Description                                               |
|--------------------|-----------------------------------------------------------|
| 🧠 Mindmap Core     | Create, edit, navigate nested ideas visually              |
| 🎛 Dashboard        | Live clock + shortcut overlay                             |
| ⌨️ Hotkeys           | Fully configurable, mapped to Ctrl/Alt bindings           |
| 🧘 Zen Mode         | Clean writing pane with cursor input                      |
| 📋 Clipboard Ops    | Copy/Cut/Paste nodes (`Ctrl+C/X/V`)                       |
| 🔍 Spotlight        | Search modal overlay (`Alt+Space`)                        |
| 📤 JSON Export      | MindTrace state export to file (`Ctrl+E`)                 |
| 🧩 Plugin-Ready     | Modular extensions like GemDrop, AutoTag, etc.            |
| 🎨 PrismX Icon      | Rendered top-right with active visual feedback            |

---

## 🧭 Keyboard Shortcuts

| Action              | Shortcut         |
|---------------------|------------------|
| Create Node         | `Ctrl+N`         |
| Copy Node           | `Ctrl+C`         |
| Cut Node            | `Ctrl+X`         |
| Paste Node          | `Ctrl+V`         |
| Toggle Dashboard    | `Ctrl+D`         |
| Toggle Mindmap      | `Ctrl+M`         |
| Enter Zen Mode      | `Ctrl+Z`         |
| Export MindTrace    | `Ctrl+E`         |
| Toggle Shortcuts    | `Ctrl+S`         |
| Spotlight Search    | `Alt+Space`      |
| Quit                | `q`              |

---

## 🧰 Getting Started

### 1. Clone and Build

```bash
git clone https://github.com/yourname/prismx.git
cd prismx
cargo build --release
