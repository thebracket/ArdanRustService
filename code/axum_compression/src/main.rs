use axum::{response::{Html, IntoResponse}, routing::get, Router};
use tower_http::compression::CompressionLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    const WAR_AND_PEACE: &str = include_str!("war_and_peace.txt");
    Html(WAR_AND_PEACE)
}
