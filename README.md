# PrismX — Modular Terminal Productivity System

## Version: v2.0.0-dev5

---

### Overview
PrismX is a modular, offline-first, plugin-capable terminal productivity framework tailored for infrastructure engineers, developers, and research-heavy workflows. It features a structured knowledge graph (mindmap), reminders, a tag-driven metadata system, plugin sandboxing, and a responsive TUI.

---

## Core Capabilities

- **Mindmap Overlay**: Visual workspace for nodes, tasks, and knowledge structures
- **Tagging Engine**: Inline, auto-tag, plugin-suggested metadata with trust layers
- **Reminder System**: Time-triggered signals for nodes, escalations, digest
- **Export Engine**: JSON, Markdown, GraphML, PDF, tag-based filtering, manifest
- **Plugin Sandbox**: Hooks, replays, trust boundaries, audit logs
- **Glossary Manager**: Role-tagged terms, aliasing, schema enforcement
- **Triage Inbox**: Task board with tag-based filtering and project breakdown
- **Audit System**: Node edit history, tag trust mutation, export integrity chain
- **TUI Layouts**: Sidebar, tag overlays, command bar, log viewer, plugin dashboard

---

## Plugin Hooks (rhai-powered)
- `on_node_create`
- `on_tag_applied`
- `on_reminder_due`
- `on_export_start`
- `on_workspace_switch`

Plugins are sandboxed and profiled. Crashes are logged but do not propagate.

---

## File Structure
```
prismx/
├── src/
│   ├── app.rs
│   ├── state.rs
│   ├── screen.rs
│   ├── config.rs
│   ├── logger.rs
│   ├── plugin/
│   ├── tag/
│   ├── ui/
│   ├── export/
│   └── zen_mode.rs
├── config.toml
├── README.md
├── logs/
├── audit/
├── exports/
└── glossary/
```

---

## CLI Tools
- `prismx export-diff`
- `prismx replay-audit`
- `prismx check-tags`
- `prismx reminder-digest`
- `prismx validate-tag-presets`

---

## Key Bindings
- `Ctrl+E` → Export Overlay
- `Ctrl+Z` → Toggle Zen Mode
- `Ctrl+L` → Toggle Log Viewer
- `Ctrl+/` → Show Shortcuts
- `Ctrl+G` → Open Tag Glossary

---

## TUI Modules
- `tag_glossary.rs` — trust overlay, inline schema edit
- `plugin_dashboard.rs` — live hook rendering, error signal
- `export_overlay.rs` — trust summary, manifest preview
- `sidebar.rs`, `status_bar.rs`, `command_bar.rs`

---

## Changelog Summary (v0.1.33 → v2.0.0-dev5)
- Fully modular plugin interface with replay and sandbox log
- Node system transitioned to UUID-backed graph
- Tagging engine refactored to support trust-based rendering
- Export engine linked to glossary, manifest, trust, preset
- All TUI components modularized and theme-ready
- Stable Audit Log / CLI Export Tools

---

## Status: ✅ QA-Ready | Tag: `v2.0.0-dev5`

For plugin authors, see `/plugin/` and `/plugin/sandbox/replay.rs`.
For glossary schema, see `/tag/trust.rs`.
