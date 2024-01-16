use axum::extract::State;
use axum::routing::get;
use axum::{async_trait, Router};
use serde::{Deserialize, Serialize};
use service_framework::{load_config, ConfigFileType, Service};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ServiceConfig {
    #[serde(default)]
    pub dburl: String,
}

#[derive(Default)]
pub struct MyService {}

impl MyService {
    async fn show_config(State(config): State<Arc<ServiceConfig>>) -> String {
        format!("{config:?}")
    }
}

#[async_trait]
impl Service for MyService {
    async fn build_web_service(&self) -> anyhow::Result<Router> {
        let settings = load_config::<ServiceConfig>(
            "APP",
            ConfigFileType::File {
                filename: String::from("config.json"),
                required: false,
            },
        )?;

        let app = Router::new()
            .route("/show_config", get(MyService::show_config))
            .with_state(settings);

        Ok(app)
    }

    fn prefix(&self) -> String {
        String::from("/api/v1")
    }
}
