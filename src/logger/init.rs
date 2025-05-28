use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use crate::logger::{level::log_filter, sinks::file_writer};

pub fn init_logger() {
    let _ = std::fs::create_dir_all("logs");
    let (non_blocking, _guard) = file_writer();

    tracing_subscriber::registry()
        .with(EnvFilter::new(log_filter()))
        .with(fmt::Layer::default().with_writer(non_blocking))
        .init();
}
