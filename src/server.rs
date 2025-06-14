use axum::{routing::get, Router, Json};
use crate::types::TelemetryReport;
use crate::metrics::gather_all_metrics;

pub fn create_app() -> Router {
    // Configure the router with a single endpoint
    Router::new().route("/metrics", get(telemetry_handler))
}

pub async fn start_server() {
    let app = create_app();

    // Start the server on port 8080
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn telemetry_handler() -> Json<TelemetryReport> {
    let report = gather_all_metrics();
    Json(report.unwrap())
}