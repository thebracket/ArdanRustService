use axum::Extension;
use axum::{response::Html, routing::get, Router};
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

struct MyCounter {
    counter: AtomicUsize,
}

#[derive(Clone)]
struct MyConfig {
    text: String,
}

#[tokio::main]
async fn main() {
    let shared_counter = Arc::new(MyCounter {
        counter: AtomicUsize::new(0),
    });
    let shared_text = Arc::new(MyConfig {
        text: "You are dynamic visitor #".to_string(),
    });

    let app = Router::new()
        .route("/", get(handler))
        .layer(Extension(shared_counter))
        .layer(Extension(shared_text));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    Extension(counter): Extension<Arc<MyCounter>>,
    Extension(config): Extension<Arc<MyConfig>>,
) -> Html<String> {
    counter.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!("<h1>{} {}</h1>",
        config.text,
        counter.counter.load(std::sync::atomic::Ordering::Relaxed)
    ))
}
