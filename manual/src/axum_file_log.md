# Logging to a File

Logging to `stdout` is often enough---if you are using `systemd` on Linux, it logs all of your standard output for you. But for larger systems, you probably want a more structured system.

> The code for this is in `code/tracing/trace_to_file`

The `tracing-appender` crate provides great log file management features. Add it with `cargo add -F tracing_appender`. Using it is really straightforward:

```rust
use tracing::info;

fn main() {
    // Setup tracing
    let file_appender = tracing_appender::rolling::hourly("test.log", "prefix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        // Build the subscriber
        .init();


    info!("Starting server");
}
```

This combines with everything else you've learned about tracing---you can customize your subscriber, add layers, etc.

By default this will roll the log file over every hour. `non_blocking` has the appender run in its own thread with a channel, and not block callers when logging---you don't have to wait for the disk.