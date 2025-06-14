use axum::{routing::get, Router, Json, Extension};
use std::sync::Arc;

use crate::types::TelemetryReport;
use crate::metrics::gather_all_metrics;
use crate::config::Config;

pub fn create_app(config: Arc<Config>) -> Router {
    // Configure the router with a single endpoint (with only GET method)
    Router::new().route(&config.metrics_path, get(telemetry_handler)).layer(Extension(config))
}

pub async fn start_server() {
    let config = Arc::new(Config::load());
    let app = create_app(config.clone());

    // Start the server on specified address & port
    let addr = format!("127.0.0.1:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server running on http://{}", addr);    

    axum::serve(listener, app).await.unwrap();
}
async fn telemetry_handler(Extension(_config): Extension<Arc<Config>>) -> Json<TelemetryReport> {
    let report = gather_all_metrics();
    Json(report.unwrap())
}