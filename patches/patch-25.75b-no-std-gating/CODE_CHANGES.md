## Code Changes

- Add `#![cfg_attr(not(feature = "std"), no_std)]`
- Replace any `std::` usage with `core::` or gated fallback
- Ensure compile success with `--no-default-features`
