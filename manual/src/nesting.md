# Nesting Multiple Routers

Unless you are building a very simple system, you probably want to offer multiple services in the same container. We'll talk about deployment options later on, for now let's make use of nesting to combine two services into one container.

You might find this useful if you have separate teams working on different parts of your application. You can easily divide responsibilities. It also makes for a cleaner, modular application.

> You can find this example in `code/simple_nested`

We'll start by making a new project as follows:

```bash
cargo new simple_nested
cargo add axum
cargo add tokio -F full
```

Then let's build a simple dual-service server:

> The code for this is in `code/rest_service/simple_nested`.

```rust
use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {

    let app = Router::new()
        .nest("/1", service_one())
        .nest("/2", service_two());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


fn service_one() -> Router {
    Router::new().route("/", get(|| async { 
        Html("Service One".to_string()) }))
}

fn service_two() -> Router {
    Router::new().route("/", get(|| async { 
        Html("Service Two".to_string()) }))
}
```

For the parent `Router`, we call `.nest` with a *parent path* and a function that returns a `Router`. These could be in different modules or crates. We also used an async closure to quickly build the response methods.

The parent path is a concatenation:

* Service one starts with a *base* of `/1`---and then `/` is appended. The *result* is `/1/`.
* Service two starts with a *base* of `/2`---and then `/` is appended. The *result* is `/2/`.

So now you can easily combine services in a single base executor. How does that work with layers and state?
