# Router Layers

You've already build a router-wide layer for authentication. Let's look at some of the pre-provided layer services and different ways of applying them.

## Providing Compression

Let's add `tower_http` with the feature `compression_full`:

```bash
cargo add tower_http -F compression_full
```

We'll also use a pre-canned file `war_and_peace.txt`---the entirety of War and Peace to demonstrate compression. Compression isn't very useful at all on tiny files, and the default `CompressionLayer` won't compress small responses (you can customize this with the `compress_when` function). So here's a complete webserver with compression enabled:

> The code is in `code/rest_service/axum_compression`

```rust
use axum::{response::{Html, IntoResponse}, routing::get, Router};
use tower_http::compression::CompressionLayer;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(handler))
        .layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    const WAR_AND_PEACE: &str = include_str!("war_and_peace.txt");
    Html(WAR_AND_PEACE)
}
```

Open a browser and load `http://localhost:3001`. Now look in the developer tools Network pane, and you will see that the response is of type `br` - Brotli compression was used to minimize the file size.

## Combining Layers into a Service

If you need a lot of layers, you can combine them with the `ServiceBuilder`. Let's also use a `CorsLayer` (from `tower_http` - it requires the feature `cors`) and a `ConcurencyLimitLayer` (which requires `cargo add tower -F limits`):

> The code for this is in `code/rest_service/axum_service_builder`

```rust
use axum::{response::{Html, IntoResponse}, routing::get, Router, http::Method};
use tower::{ServiceBuilder, limit::ConcurrencyLimitLayer};
use tower_http::{compression::CompressionLayer, cors::{CorsLayer, Any}};

#[tokio::main]
async fn main() {
    let service = ServiceBuilder::new()
        .layer(CompressionLayer::new())
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
        .       allow_origin(Any)
        )
        .layer(ConcurrencyLimitLayer::new(100));

    let app = Router::new()
        .route("/", get(handler))
        .layer(service.into_inner());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    const WAR_AND_PEACE: &str = include_str!("../../axum_compression/src/war_and_peace.txt");
    Html(WAR_AND_PEACE)
}
```

Tower and Tower_http provide a *lot* of useful layers. There are rate limiters, timeouts, load balancers, metrics, tracing. It's well worth diving into the documentation!