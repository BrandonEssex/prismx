use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};

pub fn file_writer() -> (NonBlocking, WorkerGuard) {
    let file_appender = tracing_appender::rolling::daily("logs", "prismx.log");
    tracing_appender::non_blocking(file_appender)
}
