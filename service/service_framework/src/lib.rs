mod service_config;

use anyhow::Result;
use axum::{async_trait, Router};
pub use service_config::{load_config, ConfigFileType};
use tokio::net::TcpListener;

#[macro_export]
macro_rules! run_services {
    ($listen:expr, $($service:ident),+) => {
        let services: Vec<Box<dyn Service>> = vec![
            $(Box::new($service::default()), )+
        ];
        if let Err(e) = run_webservices($listen, &services).await {
            anyhow::bail!("Unable to start webserver. {e:?}");
        }
    };
}

#[async_trait]
pub trait Service {
    async fn build_web_service(&self) -> Result<Router>;
    fn prefix(&self) -> String;
}

async fn build_router(services: &[Box<dyn Service>]) -> Result<Router> {
    let mut app = Router::new();
    for service in services {
        let web_service = service.build_web_service().await?;
        let prefix = service.prefix();

        app = app.nest(&prefix, web_service);
    }
    Ok(app)
}

pub async fn run_webservices(listen_string: &str, services: &[Box<dyn Service>]) -> Result<()> {
    let listener = TcpListener::bind(listen_string).await?;
    let app = build_router(services).await?;
    tracing::info!("Listening for web connections on {listen_string}");
    axum::serve(listener, app).await?;
    Ok(())
}
