use axum::{response::{Html, IntoResponse}, routing::get, Router, http::Method};
use tower::{ServiceBuilder, limit::ConcurrencyLimitLayer};
use tower_http::{compression::CompressionLayer, cors::{CorsLayer, Any}};

#[tokio::main]
async fn main() {
    let service = ServiceBuilder::new()
        .layer(CompressionLayer::new())
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
        .       allow_origin(Any)
        )
        .layer(ConcurrencyLimitLayer::new(100));

    let app = Router::new()
        .route("/", get(handler))
        .layer(service.into_inner());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    const WAR_AND_PEACE: &str = include_str!("../../axum_compression/src/war_and_peace.txt");
    Html(WAR_AND_PEACE)
}
