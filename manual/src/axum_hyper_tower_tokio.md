# Understanding the Service Stack

Despite being 14 lines of code (including whitespace), the `simple_http_server` example actually does quite a lot. Before we dive into expanding it, lets take a quick tour through *how* Axum is making your system function.

Axum is built on a stack of different services:

**Layer**|**Purpose**|
--|--|
**Tokio**|Provides an async runtime and executor. By default it runs one thread per core, with work-stealing. It can be configured to run single-threaded, or on a limited number of threads.|
**Hyper**|Provides HTTP services, including request/response formatting and parsing. You *can* provide services with plain hyper, but by default the full stack will be invoked.|
**Tower**|Provides a middleware service layer on top of Hyper and Tokio. Tower can be used for everything from providing timeout services to authentication layers. Tower layers and extensions can be provided to Axum applications through dependency injection.|

Let's go through an annotated version of our simple http server, making sure we understand each step:

```rust
use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    // Axum's Router type provides mappings between URLs and handlers.
    // One Router can provide multiple routes.
    // Routes are specified as a pair of data items:
    // * "/" - The relative URL to handle
    // * "get(..)" - the HTTP verb to use
    // * (handler) - a function or async closure to handle the request.
    let app = Router::new().route("/", get(handler));

    // Tokio provides a `TcpListener` type that binds an incoming TCP
    // socket. It works exactly like the TCP server example in the
    // Rust Foundations class.
    // In this case, we're listening to localhost, port 3001.
    // You can use 0.0.0.0 to listen everywhere, IPv6, etc.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    // Calling axum::serve creates an asynchronous *future* that will
    // execute Axum with the provided Router object. If you `await` it,
    // execution will continue until Axum shuts down. If you want to do
    // *other* things, use tokio::spawn to launch it and continue.
    axum::serve(listener, app).await.unwrap();
}

// The function may be named anything you want (within reason!)
// The return type is an Axum *Response* type. In this case,
// it is expecting the Html response type---which instructs the
// server to provide the appropriate MIME type.
// The Html handler is *generic*, in this case it will always
// return the string literal from inside the program.
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
```
