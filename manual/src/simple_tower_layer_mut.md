# Add a Simple Tower Layer (Mutable State)

State and layers still obey Rust's rules: you can't change shared state without locking it in some way (atomics or a lock). We've added global immutable state---let's create global mutable state.

We'll start with the `simple_tower_server` code and add to it.

> The code for this is in `code/rest_service/simple_tower_server_mut`.

```rust
use axum::{response::Html, routing::get, Router};
use axum::extract::State;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

struct MyConfig {
    counter: AtomicUsize,
}

#[tokio::main]
async fn main() {
    let shared_config = Arc::new(MyConfig {
        counter: AtomicUsize::new(0),
    });

    let app = Router::new()
        .route("/", get(handler))
        .with_state(shared_config);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(config): State<Arc<MyConfig>>) -> Html<String> {
    config.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!("<h1>You are visitor number: {}</h1>", 
        config.counter.load(std::sync::atomic::Ordering::Relaxed)
    ))
}
```

In this code we've changed:

* We're importing `AtomicUsize`
* We've replaced the config string with an atomic counter.
* We set the counter to zero.
* The handler uses `fetch_add` to atomically increment the counter.
* We use `load` to fetch the new counter value and display it. (We could just return the number from `fetch_add`)

We're using the *interior mutability* pattern we talked about it in Rust Foundations: the shared state remains immutable from the outside, but because it protects its mutable contents---it can be safely shared with an `Arc`.

Now you have the ability to share mutable and immutable state.