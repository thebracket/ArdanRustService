use tracing::info;

fn main() {
    // Setup tracing
    let file_appender = tracing_appender::rolling::hourly("test.log", "prefix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .json()
        .with_writer(non_blocking)
        .init();

    info!("Starting server");
}
