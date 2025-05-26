use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initialize global logger for PrismX.
///
/// The log level is controlled by the `PRISMX_LOG` environment variable.
/// Supported values are `debug`, `info`, and `off`.
/// Log files are written to `logs/prismx.log.YYYY-MM-DD`.
pub fn init_logger() {
    let _ = std::fs::create_dir_all("logs");

    let file_appender = tracing_appender::rolling::daily("logs", "prismx.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let level = std::env::var("PRISMX_LOG").unwrap_or_else(|_| "info".to_string());
    let filter = match level.to_lowercase().as_str() {
        "debug" => "debug",
        "off" => "off",
        _ => "info",
    };

    tracing_subscriber::registry()
        .with(EnvFilter::new(filter))
        .with(fmt::Layer::default().with_writer(non_blocking))
        .init();
}


#[macro_export]
macro_rules! log_debug {
    ($state:expr, $($arg:tt)*) => {
        if $state.debug_input_mode {
            tracing::debug!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        tracing::warn!($($arg)*);
    };
}
