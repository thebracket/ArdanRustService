use axum::{response::Html, routing::get, Router, Json};
use axum::extract::State;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

struct Counter {
    count: AtomicUsize,
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(Counter {
        count: AtomicUsize::new(0),
    });

    let app = Router::new()
        .route("/", get(handler))
        .route("/inc", get(increment))
        .with_state(counter);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn increment(State(counter): State<Arc<Counter>>) -> Json<usize> {
    println!("/inc service called");
    let current_value = counter.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Json(current_value)
}

async fn handler() -> Html<String> {
    println!("Sending GET request");
    let current_count = reqwest::get("http://localhost:3001/inc")
        .await
        .unwrap()
        .json::<i32>()
        .await
        .unwrap();

    Html(format!("<h1>Remote Counter: {current_count} </h1>"))
}
