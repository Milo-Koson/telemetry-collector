use axum::response::IntoResponse;
use axum::{Extension, Router, routing::get};
use prometheus::{Encoder, Registry, TextEncoder};
use std::sync::Arc;

use crate::config::Config;
use crate::status_metrics::{STATUS_REGISTRY, update_status_metrics};
use crate::system_metrics::{SYSTEM_REGISTRY, update_system_metrics};

pub fn create_app(config: Arc<Config>) -> Router {
    Router::new()
        .route(&config.system_metrics_path, get(system_metrics_handler))
        .route(&config.status_metrics_path, get(status_metrics_handler))
        .layer(Extension(config))
}

async fn system_metrics_handler(Extension(_config): Extension<Arc<Config>>) -> impl IntoResponse {
    update_system_metrics();
    prepare_metrics(&SYSTEM_REGISTRY)
}

async fn status_metrics_handler(Extension(_config): Extension<Arc<Config>>) -> impl IntoResponse {
    update_status_metrics();
    prepare_metrics(&STATUS_REGISTRY)
}

pub async fn start_server() {
    let config = Arc::new(Config::load());
    let app = create_app(config.clone());

    // Start the server on specified address & port
    let addr = format!("0.0.0.0:{}", config.port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!(
        "System metrics are available on http://{}{}",
        addr, config.system_metrics_path
    );
    println!(
        "Status metrics are available on http://{}{}",
        addr, config.status_metrics_path
    );

    axum::serve(listener, app).await.unwrap();
}

fn prepare_metrics(registry: &Registry) -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    let body = String::from_utf8(buffer).unwrap();

    (
        [(
            axum::http::header::CONTENT_TYPE,
            encoder.format_type().to_string(),
        )],
        body,
    )
}
