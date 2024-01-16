# Axum Spans

You might want to trace the entirety of a trip through Axum, rather than instrumenting your own code. Unfortunately, this requires a bit of boilerplate.

First of all, a working example:

```rust
use axum::{body::Body, http::Request, response::Html, routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing::info;
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

    let app =
        Router::new()
            .route("/", get(handler))
            .layer(
                TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                    let request_id = uuid::Uuid::new_v4();
                    tracing::span!(
                        tracing::Level::INFO,
                        "request",
                        method = tracing::field::display(request.method()),
                        uri = tracing::field::display(request.uri()),
                        version = tracing::field::debug(request.version()),
                        request_id = tracing::field::display(request_id)
                    )
                }),
            );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    info!("Serving Hello World");
    Html("<h1>Hello, World!</h1>")
}
```

When you run the server and connect to `http://localhost:3001`, you will see span messages for the `GET` request:

```
2024-01-16T20:48:48.444695Z  INFO ThreadId(01) axum_spans_own/src/main.rs:28: Starting server
2024-01-16T20:48:48.444845Z  INFO ThreadId(01) axum_spans_own/src/main.rs:49: listening on 127.0.0.1:3001
2024-01-16T20:48:50.355855Z  INFO ThreadId(21) request: axum_spans_own/src/main.rs:54: Serving Hello World method=GET uri=/ version=HTTP/1.1 request_id=9a3733b4-6f9c-4c39-9d29-c24abe577a83
2024-01-16T20:48:50.355929Z  INFO ThreadId(21) request: axum_spans_own/src/main.rs:35: close time.busy=66.4µs time.idle=31.1µs method=GET uri=/ version=HTTP/1.1 request_id=9a3733b4-6f9c-4c39-9d29-c24abe577a83
```

So what are we doing differently?

We've extended the `TraceLayer` to generate spans:

```rust
.layer(
    TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
        let request_id = uuid::Uuid::new_v4();
        tracing::span!(
            tracing::Level::INFO,
            "request",
            method = tracing::field::display(request.method()),
            uri = tracing::field::display(request.uri()),
            version = tracing::field::debug(request.version()),
            request_id = tracing::field::display(request_id)
        )
    }),
);
```

The `tracing` system doesn't generate its own unique IDs for requests, so we've added the UUID crate (`cargo add uuid -F v4`) for that purpose.

> Warning: generating random UUIDs isn't the fastest operation out there. Excessive tracing in this manner will slow your program down!

Then we use the `span` macro from `tracing` to specify the details of what we're logging. You could read the `request` field to customize what gets emitted.
