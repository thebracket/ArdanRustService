mod configuration;
mod db;
mod web_service;
use anyhow::Result;
use axum::{middleware, routing::{get, post}, Extension, Router};
use crate::auth::auth_layers;

pub async fn setup_service() -> Result<Router> {
    let config = configuration::BookstoreConfiguration::load()?;
    let db_pool = db::get_connection_pool(&config.db_filename).await?;

    db::perform_migrations(db_pool.clone()).await?;

    let secure_router = Router::new()
        .layer(Extension(config.clone()))
        .layer(Extension(db_pool.clone()))
        .route("/add", post(web_service::add_book))
        .route("/delete/:id", get(web_service::delete_book))
        .route("/update/:id", post(web_service::update_book))
        .route_layer(middleware::from_fn(auth_layers::require_token));

    let router = Router::new()
        .merge(secure_router)
        .route("/", get(web_service::all_books))
        .route("/:id", get(web_service::get_book))
        .layer(Extension(config))
        .layer(Extension(db_pool));

    Ok(router)
}
