## Code Changes

- ✅ Spotlight text now renders with proper foreground + padding
- ✅ Dock label width adjusted to fit: Zen, GemX, PrismX
- ✅ Top-right Prism icon restored using `canvas/prism.rs`
- ✅ Module names corrected and de-duplicated across view/state
- ✅ Removed rogue overlays or scroll render artifacts
- ❌ Disabled WASM build target temporarily (wasm/entry.rs + Cargo.toml) to prevent compile break
