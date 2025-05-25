# Patch 25.56b – Logging + Trace Support

Adds structured logging macros gated by `debug_input_mode`. Logs are routed through `log_debug!`, `log_warn!`, and `log_info!` as macros for easy injection throughout PrismX.
