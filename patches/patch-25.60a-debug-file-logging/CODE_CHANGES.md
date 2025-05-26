## Code Changes

- Adds `logging.rs` to configure `tracing_subscriber`
- Outputs to `logs/prismx.log.YYYY-MM-DD` via `tracing_appender`
- `init_logger()` called during bootstrap
- Reads `PRISMX_LOG` env var to determine verbosity
