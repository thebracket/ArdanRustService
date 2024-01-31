mod auth;
mod bookstore;
mod service_config;
use anyhow::Result;
use axum::Extension;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let service_settings = service_config::ServiceConfig::load()?;
    let auth_router = auth::setup_service().await?;
    let books_router = bookstore::setup_service().await?;

    // Listen address from configuration
    let listen_address = format!(
        "{}:{}",
        service_settings.listen_address, service_settings.listen_port
    );
    let listener = tokio::net::TcpListener::bind(&listen_address).await?;
    tracing::info!("Listening on {}", listen_address);

    // The default web server
    let static_content = ServiceBuilder::new()
        .layer(CorsLayer::very_permissive())
        .service(ServeDir::new("static_html"));

    // Build the master router
    let master_router = axum::Router::new()
        .layer(CorsLayer::very_permissive())
        .nest("/api/v1/auth", auth_router)
        .nest("/api/v1/books", books_router)
        .layer(Extension(service_settings))
        .nest_service("/", static_content);

    // Launch Axum
    axum::serve(listener, master_router).await?;

    Ok(())
}
