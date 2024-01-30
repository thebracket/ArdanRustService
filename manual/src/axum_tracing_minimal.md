# Minimal Example

Let's create a project:

```bash
cargo new axum_tracing
cd axum_tracing
cargo add tokio -F full
cargo add axum
cargo add tracing
cargo add tracing_subscriber
```

And we'll paste in a tracing example:

> The code is in `code/tracing/axum_tracing_minimal`.

```rust
use axum::{response::Html, routing::get, Router};
use tracing::info;

#[tokio::main]
async fn main() {
    // Setup default tracing
    tracing_subscriber::fmt::init();
    info!("Starting server");
    
    let app = Router::new().route("/", get(handler));

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

When you run this and visit the handler, you are greeted with various informational messages:

```
2024-01-15T17:09:08.415946Z  INFO axum_tracing_minimal: Starting server
2024-01-15T17:09:08.416072Z  INFO axum_tracing_minimal: listening on 127.0.0.1:3001
2024-01-15T17:10:10.327386Z  INFO axum_tracing_minimal: Serving Hello World
```

You can specify the output level with an environment variable, `RUST_LOG`:

```bash
RUST_LOG=error cargo run
```

Won't display anything, because we have no errors.