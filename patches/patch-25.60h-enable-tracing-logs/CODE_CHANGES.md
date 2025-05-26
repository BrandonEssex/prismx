## Code Changes

- Added `tracing::info!`, `debug!`, and `error!` to:
  - App bootstrap (module startup)
  - Input handling (hotkeys, Spotlight, mode changes)
  - Layout system (arrange, fallback, recursion)
  - Plugin loader (success, failure, path)
- Verifies `init_logger()` is triggered on `main()` startup
- Adds one line confirming log activation: `INFO PrismX logging started`
