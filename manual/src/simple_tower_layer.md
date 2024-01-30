# Add a Simple Tower Layer: State

The previous example wasn't really using Tower at all, other than inside Axum's internals. You've used Tower layers in Rust Foundations, but only skimmed the surface.

Let's make a *very simple* example of a Tower service. We'll share some configuration with our Axum handler methods.

We'll start with the `simple_http_server` code and add to it.

> The code for this is in `code/rest_service/simple_tower_server`.

```rust
use axum::{response::Html, routing::get, Router};
use axum::extract::State;
use std::sync::Arc;

struct MyConfig {
    config_string: String,
}

#[tokio::main]
async fn main() {
    let shared_config = Arc::new(MyConfig {
        config_string: "My Config String".to_string(),
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
    Html(format!("<h1>{}</h1>", config.config_string))
}
```

Let's look at what changed:

* `use axum::extract::State;` indicates that we want to use Axum's *extractor* named `State`. Extractors obtain data for incoming requests. State is a built-in service that uses Tower to allow you to attach state objects to requests.
* We've created a new type, `MyConfig`. We'll use this to store some state.
* We initialize the config and wrap it in an `Arc`. In Rust Foundations, we disucssed arc. It reference counts a single instance of a type, in a thread-safe fashion. So you will only have one `MyConfig`, it can be shared between threads---read only.
* We added `with_state` to inject the state *into* our `Router`.
* The handler now has the parameter `State(config): State<Arc<MyConfig>>`.
    * The left part `State(config)` indicates that we want to retrieve some state and refer to it as `config` in the handler.
    * The right part lists the full type we want to receive: `Arc<MyConfig>`, which will be wrapped in `State`.
* We return an `Html<String>` - we'll build the response dynamically as a string.
* We use `format` to include the contents of the state object.

So what have we achieved? We've got a global variable, shared between everything in our `Router`---but without the messiness of a global variable. The global remains protected by Rust's borrow checker---race conditions won't compile.
