use axum::{response::Html, routing::get, Router};
use axum::extract::State;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

struct MyConfig {
    counter: AtomicUsize,
}

#[tokio::main]
async fn main() {
    let shared_config = Arc::new(MyConfig {
        counter: AtomicUsize::new(0),
    });

    let app = Router::new()
        .route("/", get(handler))
        .with_state(shared_config);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(config): State<Arc<MyConfig>>) -> Html<String> {
    config.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!("<h1>You are visitor number: {}</h1>", 
        config.counter.load(std::sync::atomic::Ordering::Relaxed)
    ))
}
