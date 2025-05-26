use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn init_logger() {
    let file_appender = tracing_appender::rolling::daily("logs", "prismx.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
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
