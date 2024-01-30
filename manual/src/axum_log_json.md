# Structured Logging to JSON

You might not want freeform text, and prefer to log to a structured layout such as JSON. 

> The code for this is in `code/tracing/trace_to_json`.

You can add `json` support to `tracing_subscriber` by adding the `json` feature:

```bash
cargo add tracing_subscriber -F json
```

Then you just add one line to your tracing initializer:

```rust
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
```

And your log entries are now in JSON:

```json
{"timestamp":"2024-01-16T21:16:19.708837Z","level":"INFO","fields":{"message":"Starting server"},"target":"trace_to_json"}
```

