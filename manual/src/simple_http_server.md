# Minimal HTTP Server

Let's start by creating a new project with `cargo new simple_http_server`. This will create a project containing a `Cargo.toml` file, a `src` folder and a `main.rs` file. These are a default "hello world" program.

Now change directory to the new project, and add some dependencies:

```bash
cargo add axum
cargo add tokio -F full
```

We can change the `main.rs` file to bootstrap a very simple HTTP server, much like we did in Rust Foundations:

> This example is in the `code/rest_service/simple_http_server` project example.

```rust
use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
```

You can run this with `cargo run`, and it will serve "Hello, World!" on TCP port 3001.

This is a very minimal example, but demonstrates how Axum provides a very simple mechanism for getting up and running quickly.