# Header Extraction

You can extract all the HTTP headers:

```rust
use axum::{response::Html, routing::get, Router};
use axum::http::header::HeaderMap;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/book", get(header_extract));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn header_extract(
    headers: HeaderMap,
) -> Html<String>
{
    Html(format!("{:#?}", headers))
}
```
