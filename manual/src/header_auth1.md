# Simple Header-Based Authentication

It's really common for REST services to require additional header information. Authentication is a classic case. Let's start by looking at what it takes to read header information inside a route handler.

## Accessing Headers in Extractors

> The code is in `code/rest_service/axum_headers`.

Here's a simple program that receives the headers for a get request:

```rust
use axum::{response::Html, routing::get, Router, http::HeaderMap};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(header_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn header_handler(
    headers: HeaderMap
) -> Html<String> {
    if let Some(header) = headers.get("x-request-id") {
        Html(format!("x-request-id: {}", header.to_str().unwrap()))
    } else {
        Html("x-request-id not found".to_string())
    }
}
```

The `HeaderMap` extractor gives you full access to the headers without having to create your own handler type. In this case, if `x-request-id` is found --- we'll print it. If it isn't, we'll report that it isn't there.

Of course, it won't be there in a regular browser call (run the program and visit `localhost:3001` to prove it).

Let's write a function that submits a request with a header:

```rust
async fn make_request() {
    // Pause to let the server start up
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Make a request to the server
    let response = reqwest::Client::new()
        .get("http://localhost:3001/")
        .header("x-request-id", "1234")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{}", response);
}
```

And we'll call it just before we start serving:

```rust
tokio::spawn(make_request());
println!("listening on {}", listener.local_addr().unwrap());
axum::serve(listener, app).await.unwrap();    
```

Now the program starts, prints `x-request-id: 1234` and continues serving.

You could easily transform this into an authentication system that checks for a header (presumably checking validity). The downside of this is that now all of your authenticated requests take a `HeaderMap` and have to remember to call the test. It works, but it's not super ergonomic. Let's fix that by writing our own middleware.
