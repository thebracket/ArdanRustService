# Path Extraction

Say you want to implement an API that responds to `/book/1` and `/book/2` by returning that book's data. This can be accomplished with a *path extractor*.

> The code for this is in `code/rest_service/axum_extract/path`.

Here's a working example:

```rust
use axum::{response::Html, routing::get, Router};
use axum::extract::Path;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/book/:id", get(path_extract));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn path_extract(
    Path(id): Path<u32>,
) -> Html<String>
{
    Html(format!("Hello, {}!", id))
}
```

What's different here?

* The route includes `:id` as a placeholder identifying where the path matching should occur.
* The handler declares `Path(id): Path<u32>` as a parameter.
    * `Path(id)` means "match the Path extractor, and name the result id".
        * `Path(..)` is actually *destructuring*. You can pattern match in function parameters.
    * `Path<u32>` specifies the actually injected type. If a non-u32 type is specified, it won't be passed to the handler (you can use other types).