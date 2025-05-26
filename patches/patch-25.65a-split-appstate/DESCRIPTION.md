# Patch 25.65a – Split AppState Into Submodules

Breaks up `state/mod.rs` into manageable submodules:

- `state/core.rs`: Core AppState struct, enums, and `Default`
- `state/navigation.rs`: Movement, focus, drill/pop
- `state/edit.rs`: Node add/delete/edit actions
- `state/zen.rs`: Zen-specific logic
- `state/spotlight.rs`: Spotlight commands
- `state/history.rs`: Undo/redo system
- `state/drag.rs`: Drag/link functions
- `state/helpers.rs`: Utilities (e.g., `update_word_count`, `syntax_from_extension`)

No logic changed. Purely restructured for readability and future patch stability.
