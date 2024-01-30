use std::time::Duration;

use axum::{response::Html, routing::get, Router, http::HeaderMap};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(header_handler));

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
