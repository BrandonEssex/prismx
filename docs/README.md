# 🌌 PrismX – Terminal Productivity, Refined

**Version:** `v11.0.0+`  
**Codename:** *X Shard Horizon*  
**Status:** Actively Patched (Current: `patch-25.74b`)  
**Minimum Terminal Size:** 100x30 (Ratatui-optimized)  
**Languages:** Rust + Crossterm + Optional WASM Plugin Targets  

---

## 🧠 What is PrismX?

**PrismX** is a modular, keyboard-native productivity system designed for terminal power users, system architects, engineers, and creators. Inspired by tools like Obsidian, Alfred, and Tana—but built entirely in the terminal—PrismX combines realtime mindmapping, writing, tagging, command routing, and extensible plugins with a cohesive visual identity and blazing-fast performance.

---

## 🧩 Core Modules

| Module     | Hotkey       | Description |
|------------|--------------|-------------|
| **GemX**   | `Ctrl+P/W/Q` | Mindmap with tidy-tree layout, zoom, drag, and collapse logic |
| **Zen**    | `Ctrl+R`     | Journal with hidden UI, breathing visuals, scroll mode, and Markdown |
| **Triage** | `Ctrl+Y`     | Live-updating tag and priority tracker with debug feed and filter bar |
| **Spotlight** | `Alt+Space` | Alfred-style fuzzy search and command launcher |
| **Settings** | `Ctrl+.`    | Toggle panel with theme previews and UX options |
| **Plugins**  | Auto-loaded | Optional modules (Countdown, Pomodoro, Calendar, etc.) with manifest-based mounting |

---

## 🔮 Major Features

### 🧠 Mindmapping (GemX)
- Tidy horizontal tree layout with spacing and zoom
- `Ctrl+W` Drill into node subtree, `Ctrl+Q` Pop back
- Drag/drop with fallback placement logic
- Zoom-to-cursor and scroll synchronization
- Auto-arrange (void-rs parity) with sibling spacing
- Breadcrumb navigation and hotkey-driven node insertion

### ✍️ Zen Writing & Journal Mode
- Markdown editing with timestamped journal blocks
- Scroll view for daily timeline review
- Breathing indicator and entry fade animation
- Left/right hidden panels for font and file operations
- Togglable classic vs. journal modes

### 🏷️ Triage Priority Tracking
- Tag bar for filtering live updates
- Automatic triage for tagged entries (`#TRITON`, `#FOCUS`, etc.)
- Planned dashboard integration
- Debug overlay feed

### 🔦 Spotlight Search
- Alfred-like fuzzy search with `tag:` and `node:` prefixes
- Predictive ghost-text input
- Searchable history (via `Ctrl+H`) and keyboard navigation
- Extensible `ActionRegistry` for plugin/command hooks

### ⚙️ Settings Panel
- Theme toggle, animation enable/disable, font controls
- Experimental settings and keyboard overlay planned
- Always accessible (`Ctrl+.`)

---

## 🧩 Plugin Architecture

| Plugin Type   | Example        | Integration Style |
|---------------|----------------|-------------------|
| **Modules**   | Journal, Calendar | Load as full screen panels |
| **Panels**    | Countdown, Pomodoro | UI overlay with theme-aware beams |
| **Commands**  | `/backup`, `/rebuild` | Spotlight-command routed |

- All plugins follow a manifest-based interface
- Native `cdylib` and future `WASM` plugin support
- Shared beam-based visual identity

---

## 🔐 Planned Features & Roadmap

| Feature         | Status      | Description |
|-----------------|-------------|-------------|
| **ShardVault**  | 🔜 Planned  | Encrypted workspace containers with multi-vault sync |
| **Federation**  | 🔜 Planned  | Workspace sharding across trusted clients with opt-in merge behavior |
| **RoutineForge**| 🔜 Planned  | Schedule and habit logging system |
| **Shortcut Overlay** | 🔜 Planned | Context-aware hotkey viewer tied to module state |
| **Live Plugin Dashboard** | 🔜 Planned | Visual plugin status, timers, TrustScore metrics |
| **WASM Preview** | 🔜 Planned | Compile and run plugins or modules in WASM sandbox for browser-native PrismX |

---

## 🧰 Developer Features

- Modular `mod.rs` refactors with isolated layout logic
- Structured debug logs via `tracing`
- Patch-driven development (`patches/patch-*`)
- Codex AI integrations for safe, reviewed code injection
- Hot-reload safe (`Ctrl+Space` toggles modules without visual glitch)

---

## 📁 State Management

- Layouts persisted across sessions (scroll, zoom, collapse)
- TOML and JSON support for snapshots
- Patch-aware content hash versioning

---

## 🚀 Get Started

1. `cargo build --release`
2. Run: `./target/release/prismx`
3. Toggle modules: `Ctrl+Space`
4. Begin mindmapping: `Ctrl+N`, `Ctrl+B`, `Ctrl+P`
5. Journal away: `Ctrl+R`

---

## 📣 Activation Phrases

To resume the full project context in ChatGPT:

```bash
# Genesis Setup
Resume PrismX from Genesis Log

# Current Status
Continue PrismX from Exodus Log
📜 License

MIT License © PrismX Contributors
