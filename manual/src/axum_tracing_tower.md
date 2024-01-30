# Logging Axum/Tower

Maybe we'd also like to log everything the webserver itself is doing (that can be a LOT of data). Let's add one more dependency:

```bash
cargo add tower_http -F trace
```

> The code for this is in `code/tracing/axum_tracing_tower`.

Then we have to add two lines of code:

```rust
use axum::{response::Html, routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    // Setup default tracing
    tracing_subscriber::fmt::init();
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

async fn handler() -> Html<&'static str> {
    info!("Serving Hello World");
    Html("<h1>Hello, World!</h1>")
}
```

We've added `TraceLayer` and a `use tower_http::trace::TraceLayer` statement. Now if you run the program with `RUST_LOG=debug cargo run` you will see *everything*:

```
2024-01-15T17:45:34.191361Z  INFO axum_tracing_tower: Starting server
2024-01-15T17:45:34.191526Z  INFO axum_tracing_tower: listening on 127.0.0.1:3001
2024-01-15T17:45:38.054710Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_request: started processing request
2024-01-15T17:45:38.054756Z  INFO request{method=GET uri=/ version=HTTP/1.1}: axum_tracing_tower: Serving Hello World
2024-01-15T17:45:38.054783Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=0 ms status=200
```

> Note that you can add `TraceLayer` to individual routers if you want to only log a specific portion of what's going on.
