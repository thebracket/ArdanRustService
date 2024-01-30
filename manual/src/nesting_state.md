# Nested Routers with State

You can add `State` and layers to nested services. The only potential surprise is that `Router`s don't see one another's state. You *can* inject the same state into multiple routers if you want to (and `Arc` is designed for that sort of thing)---but if you don't add it to a router, that router doesn't offer it to its handlers.

Let's demonstrate this by adding some state to our previous example.

> The code for this is in `code/rest_service/simple_nested_state`

```rust
use std::sync::Arc;
use axum::{response::Html, routing::get, Router, extract::State};

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

struct MyState {
    n: i32,
}

fn service_one() -> Router {
    let state = Arc::new(MyState { n: 1 });
    Router::new()
        .route("/", get(handler))
        .with_state(state)
}

fn service_two() -> Router {
    let state = Arc::new(MyState { n: 2 });
    Router::new()
        .route("/", get(handler))
        .with_state(state)
}

async fn handler(State(my_state): State<Arc<MyState>>) -> Html<String> {
    Html(format!("State: {}", my_state.n))
}
```

So we're demonstrating that each `Router` has completely independent state: each has its own instance of `MyState`. The same thing works with extension layers. Your nested services can operate independently of one another.

