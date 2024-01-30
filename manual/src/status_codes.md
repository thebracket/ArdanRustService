# Returning Status Codes

So far, we've been lazy. We've handled errors with `unwrap` (which won't crash Axum!) and not
used the HTTP status code system at all.

Let's start rectifying that by learning how to return status codes from web handlers.

> The code for this is in `code/rest_service/status_codes`

```rust
use axum::{routing::get, Router, http::StatusCode};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> StatusCode {
    StatusCode::SERVICE_UNAVAILABLE
}
```

You can simply return a `StatusCode` enumeration entry from a function to return that as the HTTP status code.
This won't display a nice error message to the user - but for REST services it lets you indicate that something
went wrong.

> It's worth thinking about how much information you want to return in your error messages, vs how much you log. You don't always want to reveal the inner workings of your system!


