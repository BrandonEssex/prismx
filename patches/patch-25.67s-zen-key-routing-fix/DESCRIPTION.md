# Patch 25.67s – Reinstate Zen Key Routing

Fixes recent regression where typing in Zen no longer creates entries. Ensures proper dispatch to `zen::editor::handle_key()` only when in Compose + Write mode.
