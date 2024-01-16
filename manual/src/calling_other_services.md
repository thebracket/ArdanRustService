# Calling Other Services with Hyper

So far, we've used the Tower and Axum layers focusing on setting up routes,
nesting services, and dependency injection. How about calling other web
services?

You can use `hyper` directly, but it is complicated---it supports just about
everything you could ever want to do with HTTP. The `Reqwest` crate uses
Hyper under the hood, but offers a much more friendly interface. So let's  
add it to our program as a dependency:

```bash
cargo add reqwest -F json
```

Now we'll combine some of what we've learned to make a webserver with two routes and
some shared state. A counter, incremented with the `/inc` route and a root (`/`) services
that uses Reqwest to call the `/inc` service and display the current counter value.

> The code for this is in `code/hyer_client`

```rust
use axum::{response::Html, routing::get, Router, Json};
use axum::extract::State;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

struct Counter {
    count: AtomicUsize,
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(Counter {
        count: AtomicUsize::new(0),
    });

    let app = Router::new()
        .route("/", get(handler))
        .route("/inc", get(increment))
        .with_state(counter);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn increment(State(counter): State<Arc<Counter>>) -> Json<usize> {
    println!("/inc service called");
    let current_value = counter.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Json(current_value)
}

async fn handler() -> Html<String> {
    println!("Sending GET request");
    let current_count = reqwest::get("http://localhost:3001/inc")
        .await
        .unwrap()
        .json::<i32>()
        .await
        .unwrap();

    Html(format!("<h1>Remote Counter: {current_count} </h1>"))
}
```

Congratulations! You can now call other REST services from within your REST service.

