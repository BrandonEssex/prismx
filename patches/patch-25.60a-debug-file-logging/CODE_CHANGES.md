## Code Changes

- Add `tracing` and `tracing-subscriber`
- Create `logging.rs` with `init_logger()` function
- Log to `logs/prismx.log` in DEBUG mode
- Replace `eprintln!` and `println!` with:
  ```rust
  tracing::debug!("...");
  tracing::warn!("...");
  tracing::error!("...");
Disable on-screen spam unless debug toggled
