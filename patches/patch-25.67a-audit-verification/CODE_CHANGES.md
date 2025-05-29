## Code Changes

- `scripts/ci/audit-integrity.sh`:
  - Greps for expected code signatures from past patches
  - Confirms presence of `init_logger`, plugin loaders, patch toggles
  - Verifies enums or fields expected from Zen features

- `src/bin/audit.rs`:
  - CLI to run validations from Rust
  - Flags missing initializers or regressions
