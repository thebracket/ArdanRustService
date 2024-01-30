use axum::{response::Html, routing::get, Router};
use tracing::info;

#[tokio::main]
async fn main() {
    // Setup default tracing
    tracing_subscriber::fmt::init();
    info!("Starting server");
    
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    info!("Serving Hello World");
    Html("<h1>Hello, World!</h1>")
}
