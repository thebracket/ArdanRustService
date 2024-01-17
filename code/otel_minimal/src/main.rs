use axum::{body::Body, http::Request, response::Html, routing::get, Router};
use tower_http::trace::TraceLayer;
use opentelemetry::{global, KeyValue, trace::TraceError, logs::LogError};
use opentelemetry_otlp::{WithExportConfig, ExportConfig};
use opentelemetry_sdk::{propagation::TraceContextPropagator, runtime, Resource, metrics::MeterProvider, logs::Config};
use tracing::{info, instrument, level_filters::LevelFilter, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use opentelemetry_sdk::trace as sdktrace; // To avoid name conflicts

fn init_tracer(otlp_endpoint: &str) -> Result<sdktrace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otlp_endpoint),
        )
        .with_trace_config(
            sdktrace::config().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "hello_world",
            )])),
        )
        .install_batch(runtime::Tokio)
}

fn init_metrics(otlp_endpoint: &str) -> opentelemetry::metrics::Result<MeterProvider> {
    let export_config = ExportConfig {
        endpoint: otlp_endpoint.to_string(),
        ..ExportConfig::default()
    };
    opentelemetry_otlp::new_pipeline()
        .metrics(runtime::Tokio)
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_export_config(export_config),
        )
        .with_resource(Resource::new(vec![KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
            "hello_world",
        )]))
        .build()
}

fn init_logs(otlp_endpoint: &str) -> Result<opentelemetry_sdk::logs::Logger, LogError> {
    opentelemetry_otlp::new_pipeline()
        .logging()
        .with_log_config(
            Config::default().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "hello_world",
            )])),
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otlp_endpoint.to_string()),
        )
        .install_batch(runtime::Tokio)
}

#[tokio::main]
async fn main() {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let otlp_endpoint = "http://localhost:4317";

    let tracer = init_tracer(&otlp_endpoint).unwrap();

    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = tracing_subscriber::registry()
        .with(LevelFilter::from_level(Level::DEBUG))
        .with(telemetry_layer);

    subscriber.init();

    let _meter_provider = init_metrics(&otlp_endpoint);
    let _log_provider = init_logs(&otlp_endpoint);

    let app = Router::new()
        .route("/", get(handler))
        .layer(
                TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                    let request_id = uuid::Uuid::new_v4();
                    tracing::span!(
                        tracing::Level::INFO,
                        "request",
                        method = tracing::field::display(request.method()),
                        uri = tracing::field::display(request.uri()),
                        version = tracing::field::debug(request.version()),
                        request_id = tracing::field::display(request_id)
                    )
                }),
            );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[instrument(level = "info")]
async fn handler() -> Html<&'static str> {
    info!("Saying Hello");
    Html("<h1>Hello, World!</h1>")
}
