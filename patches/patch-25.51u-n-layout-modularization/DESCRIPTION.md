# Patch 25.51u-n – Layout Modularization

Moves `recalculate_roles()` and fallback promotion logic into the `layout/` module. This prepares for better testability and isolates complex layout logic from AppState.
