## Code Changes

- Created `state/core.rs` and moved:
  - `AppState`, `DockLayout`, `ZenTheme`, `ZenSyntax`, `SimInput`, etc.
  - `Default for AppState`

- Created and moved logic to:
  - `navigation.rs`: `move_focus_*`, `drill_selected`, `pop_stack`
  - `edit.rs`: `add_child_node`, `add_sibling_node`, `delete_node`, `toggle_collapse`
  - `zen.rs`: Zen editor, file I/O, journal, theme, word count
  - `spotlight.rs`: Spotlight execution, history
  - `history.rs`: `push_undo`, `undo`, `redo`
  - `drag.rs`: `start_drag`, `complete_drag`, `start_link`, `complete_link`
  - `helpers.rs`: `update_active_label`, `delete_last_char`, `syntax_from_extension`

- Rewired `mod.rs` to re-export these logically grouped files.
- All functionality preserved exactly. No behavior changes.
