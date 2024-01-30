# Error Handling with IntoResponse

Axum's `IntoResponse` type includes a conversion from Rust's `Result` type. Returning an error response with a `Result` won't cause an unwrap and crash---it will cause Axum to return the type inside the error type.

*So why is that helpful?*

Now we can start to write handlers that use Rust's regular error-handling syntax, and fall back to a status code if something went wrong.

## Fallible Handlers with StatusCode Fallback

> The code for this is in `code/rest_service/axum_error-handling`.

Let's look at some example code:

```rust
use axum::{response::{Html, IntoResponse}, routing::get, Router, http::StatusCode, Json};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/html", get(html_handler))
        .route("/json", get(json_handler))
        .route("/error", get(error_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn html_handler() -> Result<impl IntoResponse, StatusCode> {
    Ok(Html("<h1>Hello, World!</h1>"))
}

async fn json_handler() -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(32))
}

async fn error_handler() -> Result<impl IntoResponse, StatusCode> {
    if 1 == 2 {
        Ok(Html("<h1>Never happens</h1>"))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}
```

All of our handlers now have the same return type:

```rust
Result<impl IntoResponse, StatusCode>
```

And now we're able to wrap our responses in the familiar Rust syntax for indicating whether a function succeeded or not:

```rust
return Ok(Html("<h1>Hello, World!</h1>"));
return Err(StatusCode::SERVICE_UNAVAILABLE);
```

We're most of the way there! Now how about handling the `?` operator for easy handling? We can't use a "naked" `?` unless we're calling a function that returns a `StatusCode`. But we can use `map_err` to handle errors:

```rust
async fn maybe_error() -> Result<impl IntoResponse, StatusCode> {
    let start = std::time::SystemTime::now();
    let seconds_wrapped = start
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .as_secs() % 3;
    let divided = 100u64.checked_div(seconds_wrapped).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(divided))
}
```

This function obtains the current time, and transforms it into a Unix timestamp. Then we take the modulus with 3 --- so every 3 seconds the number will be 0. Then we perform a checked division---every 3 seconds, the service will fail.

Notice that we're using `?` and `map_err`---just like we did in Rust Foundations.

This allows you to write fluid Rust code, making use of Rust's error handling.

## Giving More Detail

But what if you want to display more detail in your error message? Axum let's you define *tuples* for your errors---containing various parts of the response (you can apply headers with `HeaderMap`, for example). Here's a simple example:

```rust
async fn maybe_error_verbose() -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let start = std::time::SystemTime::now();
    let seconds_wrapped = start
        .duration_since(std::time::UNIX_EPOCH,)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Time went backwards!"))?
        .as_secs() % 3;
    let divided = 100u64.checked_div(seconds_wrapped).ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Division by zero"))?;
    Ok(Json(divided))
}
```

You can replace `&'static str` with `String` and use `format!` for more specific errors.