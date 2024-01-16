use axum::{response::Html, routing::get, Router};
use axum::extract::State;
use std::sync::Arc;

struct MyConfig {
    config_string: String,
}

#[tokio::main]
async fn main() {
    let shared_config = Arc::new(MyConfig {
        config_string: "My Config String".to_string(),
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
    Html(format!("<h1>{}</h1>", config.config_string))
}
