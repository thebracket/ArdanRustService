# Serving Static Content with Tower

> The code for this is in `code/rest_service/static_content`.

We'll need to add a dependency for Tower itself and the Tower HTTP helper system:

```bash
cargo add tower -F util
cargo add tower-http -F fs
```

The following code will provide a static handler that serves files in the `web` directory, if the filename exists and the provided URL was not already matched by a handler:

```rust
use axum::{response::{Html, IntoResponse}, routing::get, Router, http::StatusCode};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/static", get(static_handler))
        .fallback_service(ServeDir::new("web"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn static_handler() -> Result<impl IntoResponse, StatusCode> {
    Ok(Html("<h1>Hello, World!</h1>"))
}
```

To give it something to serve, in the `static_content` project directory we've added a directory named `web`. In that directory, there's a file named `demo.html`:

```html
<!DOCTYPE html>
<html>
<head>
    <title>demo</title>
</head>
<body>
    <h1>demo</h1>
    <p>demo</p>
</html>
```

Now test it by going to: `http://localhost:3001/demo.html` in a browser.

## So What's Happening Here?

Look at the router:

```rust
let app = Router::new()
    .route("/static", get(static_handler))
    .fallback_service(ServeDir::new("web"));
```

Axum provides the concept of a "fallback service"---a service to try if none of the direct routes matched. We've attached `ServeDir`, which is a Tower middleware service defined in `tower_http`. That service receives the Hyper-formatted HTTP information about unhandled connections and tries to serve them.
