use axum::{response::{Html, IntoResponse}, routing::get, Router, http::StatusCode, Json};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/html", get(html_handler))
        .route("/json", get(json_handler))
        .route("/error", get(error_handler))
        .route("/maybe", get(maybe_error))
        .route("/verbose", get(maybe_error_verbose));

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

async fn maybe_error() -> Result<impl IntoResponse, StatusCode> {
    let start = std::time::SystemTime::now();
    let seconds_wrapped = start
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .as_secs() % 3;
    let divided = 100u64.checked_div(seconds_wrapped).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(divided))
}

async fn maybe_error_verbose() -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let start = std::time::SystemTime::now();
    let seconds_wrapped = start
        .duration_since(std::time::UNIX_EPOCH,)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Time went backwards!"))?
        .as_secs() % 3;
    let divided = 100u64.checked_div(seconds_wrapped).ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Division by zero"))?;
    Ok(Json(divided))
}