# Simple Header-Based Auth with Middleware

Axum combines with Tower to let you build your own middleware---that is code that runs as part of the routing service, intercepting requests on their way in.

> The code for this is in `code/rest_service/axum_header_layer`.

You can write middleware as a function:

```rust
async fn auth(
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {    
    if let Some(header) = headers.get("x-request-id") {
        // Validate the header
        if header.to_str().unwrap() != "1234" {
            return Err((StatusCode::UNAUTHORIZED, "invalid header".to_string()));
        }
    }

    Ok(next.run(req).await)
}
```

Middleware functions accept extractors just like handler functions. In this case, we're extracing the headers. Returning an error stops subsequent layers from executing, and you can return an error. Alternatively, calling `next.run(req).await` passes control to the next handler in the chain.

You register middleware for all layers in a `Router` as follows:

```rust
let app = Router::new()
    .route("/", get(header_handler))
    .route_layer(middleware::from_fn(auth));
```

> If you want different routes to use different middleware, you need to define multiple `Router`s and nest them.

So here's an expanded version of the previous code. We're calling the API twice, once with a valid header and once without. We're using `println` to show that the handler never executes for the invalid header:

```rust
use std::time::Duration;
use axum::{
    http::{HeaderMap, StatusCode},
    middleware::{Next, self},
    response::{Html, IntoResponse},
    routing::get,
    Router, extract::Request,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(header_handler))
        .route_layer(middleware::from_fn(auth));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    tokio::spawn(make_request());

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

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

    let response = reqwest::Client::new()
        .get("http://localhost:3001/")
        .header("x-request-id", "bad")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{}", response);
}

async fn auth(
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {    
    // TODO: Fix this to not succeed when there isn't a header
    if let Some(header) = headers.get("x-request-id") {
        // Validate the header
        if header.to_str().unwrap() == "1234" {
            return Ok(next.run(req).await);
        }
    }

    Err((StatusCode::UNAUTHORIZED, "invalid header".to_string()))
}

async fn header_handler(headers: HeaderMap) -> Html<String> {
    if let Some(header) = headers.get("x-request-id") {
        Html(format!("x-request-id: {}", header.to_str().unwrap()))
    } else {
        Html("x-request-id not found".to_string())
    }
}
```

That's an improvement---all authentication is now self-contained. It's still really messy to require that the handler *also* look at the headers and obtain the data. The good news is that we can fix that, too.