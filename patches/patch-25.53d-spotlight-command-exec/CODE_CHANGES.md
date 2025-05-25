## Code Changes

### 1. In Spotlight input handler (on Enter key):

```rust
let cmd = state.spotlight_input.trim();
match cmd {
    "/triage" => {
        state.mode = Mode::Triage;
        state.spotlight = false;
    }
    "/zen" => {
        state.mode = Mode::Zen;
        state.spotlight = false;
    }
    "/settings" => {
        state.mode = Mode::Settings;
        state.spotlight = false;
    }
    "/gemx" => {
        state.mode = Mode::GemX;
        state.spotlight = false;
    }
    _ => {
        // unknown or ignored
    }
}
Optionally push to spotlight_history here as well.
2. Reset spotlight state after execution:
state.spotlight_input.clear();
state.spotlight_just_opened = false;
