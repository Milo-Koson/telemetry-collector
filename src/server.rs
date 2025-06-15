use axum::response::IntoResponse;
use axum::{Extension, Router, routing::get};
use prometheus::{Encoder, TextEncoder, gather};
use std::sync::Arc;

use crate::config::Config;
use crate::metrics::update_metrics;

pub fn create_app(config: Arc<Config>) -> Router {
    // Configure the router with a single endpoint (with only GET method)
    Router::new()
        .route(&config.metrics_path, get(telemetry_handler))
        .layer(Extension(config))
}

pub async fn start_server() {
    let config = Arc::new(Config::load());
    let app = create_app(config.clone());

    // Start the server on specified address & port
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server running on http://{}{}", addr, config.metrics_path);

    axum::serve(listener, app).await.unwrap();
}
async fn telemetry_handler(Extension(_config): Extension<Arc<Config>>) -> impl IntoResponse {
    update_metrics();
    let metric_families = gather();

    // Encoder to convert metrics to text format
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let body = String::from_utf8(buffer).unwrap();

    // Send the response with the correct Content-Type
    (
        [(
            axum::http::header::CONTENT_TYPE,
            encoder.format_type().to_string(),
        )],
        body,
    )
}
