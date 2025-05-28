## Code Changes

- Add `#![cfg_attr(not(feature = "std"), no_std)]`
- Replace `std::collections` with `alloc`
- Separate TUI/render code behind `#[cfg(std)]`
