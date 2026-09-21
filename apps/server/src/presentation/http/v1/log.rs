use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};

pub fn init_logger() -> (NonBlocking, WorkerGuard) {
    let file_appender = tracing_appender::rolling::hourly("logs", "drydrop-server.log");
    tracing_appender::non_blocking(file_appender)
}
