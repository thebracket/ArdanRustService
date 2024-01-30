# Timing Spans

You saw in *Rust Foundations* that you can add spans to functions to track execution time. This continues to work inside Axum. It doesn't time the whole `request -> layers -> handler -> layers -> response` cycle---just your hanlder. For locating performance issues in your code, that can be just fine.

Spans work as they did before:

> The code for this is in `code/tracing/axum_spans_own`.

```rust
use axum::{response::Html, routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing::{info, instrument};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    // Setup tracing
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Include per-span timings
        .with_span_events(FmtSpan::CLOSE)
        // Build the subscriber
        .finish();

    // Set the subscriber as the default
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Starting server");

    let app = Router::new()
        .route("/", get(handler))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[instrument]
async fn handler() -> Html<&'static str> {
    info!("Serving Hello World");
    Html("<h1>Hello, World!</h1>")
}
```

We've added span events to the tracing subscriber, and used `#[instrument]` to wrap our handler in a span. The server will now show you execution timings:

```
2024-01-16T20:37:34.281844Z  INFO ThreadId(01) axum_spans_own/src/main.rs:28: Starting server
2024-01-16T20:37:34.281973Z  INFO ThreadId(01) axum_spans_own/src/main.rs:38: listening on 127.0.0.1:3001
2024-01-16T20:37:37.201389Z  INFO ThreadId(21) handler: axum_spans_own/src/main.rs:44: Serving Hello World
2024-01-16T20:37:37.201445Z  INFO ThreadId(21) handler: axum_spans_own/src/main.rs:42: close time.busy=60.9µs time.idle=12.6µs
```
