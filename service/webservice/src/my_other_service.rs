use axum::extract::State;
use axum::routing::get;
use axum::{async_trait, Router};
use serde::{Deserialize, Serialize};
use service_framework::{load_config, ConfigFileType, Service};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OtherConfig {
    #[serde(default)]
    pub some_text: String,
}

#[derive(Default)]
pub struct OtherService {}

impl OtherService {
    async fn show_config(State(config): State<Arc<OtherConfig>>) -> String {
        format!("{config:?}")
    }
}

#[async_trait]
impl Service for OtherService {
    async fn build_web_service(&self) -> anyhow::Result<Router> {
        let settings = load_config::<OtherConfig>(
            "APP",
            ConfigFileType::File {
                filename: String::from("config.json"),
                required: false,
            },
        )?;

        let app = Router::new()
            .route("/show_config", get(OtherService::show_config))
            .with_state(settings);

        Ok(app)
    }

    fn prefix(&self) -> String {
        String::from("/api/v2")
    }
}
