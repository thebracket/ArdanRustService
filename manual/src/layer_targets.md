# Selectively Applying Layers

You may not want to apply a layer across an entire service. Having to make a new nested router just for one handler to gain some functionality can get messy. Axum provides some flexibility.

Functions for applying layers differently:

* You can apply a layer to a whole `Router` with `route_layer` and `.layer` in the router builder.
* You can annotate individual handlers with the `layer` command.

Let's work through each of these, including learning how to procedurally route data based on the request.

> The code for this is in `code/axum_merge`.

Here's a very simple example:

```rust
use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let other = Router::new().route("/", get(handler2));
    let app = Router::new().route("/", get(handler)).merge(other);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn handler2() -> Html<&'static str> {
    Html("<h1>Hello, World 2!</h1>")
}
```