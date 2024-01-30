# Multiple States

Axum has a limitation that you can only have *one* state per `Router`. So now you face a choice:

1. You can group everything into a single `State` object and use careful interior mutability.
2. You can replace `State` with an *Extension Layer*.

State is deliberately lightweight, and doesn't cause much extra processing. Layers are a little slower, because they invoke the Tower middleware system directly.

> Tip: Use state if you only need one. Extensions aren't *that* heavy, it will still perform very well.

Let's rewrite out previous example using extensions, and have two extensions:

> The code for this example is in `code/rest_service/simple_tower_server_multi_state`.

```rust
use axum::Extension;
use axum::{response::Html, routing::get, Router};
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

struct MyCounter {
    counter: AtomicUsize,
}

#[derive(Clone)]
struct MyConfig {
    text: String,
}

#[tokio::main]
async fn main() {
    let shared_counter = Arc::new(MyCounter {
        counter: AtomicUsize::new(0),
    });
    let shared_text = Arc::new(MyConfig {
        text: "You are dynamic visitor #".to_string(),
    });

    let app = Router::new()
        .route("/", get(handler))
        .layer(Extension(shared_counter))
        .layer(Extension(shared_text));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    Extension(counter): Extension<Arc<MyCounter>>,
    Extension(config): Extension<Arc<MyConfig>>,
) -> Html<String> {
    counter.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!("<h1>{} {}</h1>",
        config.text,
        counter.counter.load(std::sync::atomic::Ordering::Relaxed)
    ))
}
```

In addition to the obvious change (two states), we've replaced:

* The `.with_state` call is now `.layer(Extension(..))` in the route definition.
* Each `Extension` is injected into the `handler` with `Extension(name): Extension<Arc<type>>`.
* We used both extensions to create the output.

