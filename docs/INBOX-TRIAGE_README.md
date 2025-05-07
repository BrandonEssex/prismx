# 📥 PrismX Inbox/Triage Module

This module provides a powerful, GTD-inspired Inbox and Triage system for PrismX. It allows you to collect, organize, prioritize, and process tasks seamlessly within a terminal-based interface.

---

## 📁 Module Features

- ✅ **GTD-Style Inbox**: Rapid capture of tasks using `Ctrl+Alt+N`.
- ✅ **Triage Mode (Ctrl+T)**: Focused UI for prioritizing, tagging, assigning, and archiving tasks.
- ✅ **Persistent JSON Storage**: Automatically saves to `inbox.json`, with error recovery and backup.
- ✅ **Keyboard Navigation**: TUI-optimized; arrow keys and shortcut-driven interaction.
- ✅ **Robust Logging**: Extreme debug logging and fault recovery in `inbox_storage.rs`.

---

## 🔑 Keyboard Shortcuts

| Shortcut        | Action                                       |
|----------------|----------------------------------------------|
| `Ctrl+T`        | Toggle triage view                           |
| `Ctrl+Alt+N`    | Create a new inbox task                      |
| `Ctrl+D`        | Archive selected task                        |
| `T`             | Edit tags for selected task                  |
| `P`             | Set task priority                            |
| `A`             | Assign selected task                         |
| `Ctrl+Q`        | Exit triage view                             |

---

## 🧱 Data Schema (`inbox.json`)

Each task in `inbox.json` follows this structure:

```jsonc
{
  "id": "uuid",
  "title": "Write README for inbox module",
  "description": "Summarize all features, shortcuts, and structure",
  "shard": "work",
  "tags": ["documentation", "high-priority"],
  "priority": "high",
  "status": "inbox",
  "assigned_to": "elijah",
  "created": "2025-05-07T01:32:00Z",
  "modified": "2025-05-07T01:33:00Z"
}
📂 File Structure

src/
├── inbox.rs              # Task model and logic (create, tag, triage, assign)
├── actions.rs            # Action enum extended with inbox operations
├── state.rs              # AppState integration of InboxState + I/O hooks
├── screen.rs             # Routes actions to state and draws appropriate views
├── view_triage.rs        # Renders TUI triage view
└── storage/
    └── inbox_storage.rs  # JSON load/save with corruption recovery

assets/
└── inbox.json            # Persistent inbox task list (auto-created if missing)
🧪 Testing & Logging

✅ Automatic backups (inbox.json.bak.<timestamp>) if JSON is corrupted
✅ Full test coverage of:
Task creation, tagging, assignment
JSON read/write, corrupted file fallback
UI rendering and action dispatch
To run tests:

cargo test --package prismx
🔧 Future Enhancements

Multi-user inbox routing
Automations based on tags or shard
Customizable keybindings
TUI filters and sorting enhancements
🧠 Authors & Maintainers

Module designed and integrated by: #GENESIS (PrismX QA/Integration Bot)
Project Lead: blck