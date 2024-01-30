# Query Extraction

You can also extract the query string as a set of names and values:

> The code for this is in `code/rest_service/axum_extract_query`.

```rust
use std::collections::HashMap;

use axum::{response::Html, routing::get, Router};
use axum::extract::Query;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/book", get(query_extract));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn query_extract(
    Query(params): Query<HashMap<String, String>>,
) -> Html<String>
{
    Html(format!("{:#?}", params))
}
```

Once again, we've used an extractor---this time `Query`. We've asked to extract query parameters into a `HashMap` of strings, and just printed the result.