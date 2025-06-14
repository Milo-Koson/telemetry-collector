use telemetry_collector::config::Config;

use std::sync::Arc;
use tower::ServiceExt;
use http::Request;
use tokio::time::{timeout, Duration};

async fn get_metrics_response() -> http::Response<axum::body::Body> {
    let config = Arc::new(Config::load());
    let app = telemetry_collector::server::create_app(config);

    let request = Request::builder()
        .uri("/metrics")
        .body(axum::body::Body::empty())
        .unwrap();

    app.oneshot(request).await.unwrap()
}

/// Verify that the endpoint returns a 200 status code
#[tokio::test]
async fn test_metrics_endpoint_status_ok() {
    let response = get_metrics_response().await;
    assert_eq!(response.status(), 200);
}

/// Verify that the response is in a valid JSON format
#[tokio::test]
async fn test_metrics_endpoint_response_json() {
    let response = get_metrics_response().await;
    assert_eq!(response.status(), 200);

    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice::<telemetry_collector::types::TelemetryReport>(&body).unwrap();
}

#[tokio::test]
async fn testing_endpoint_methods() {
    let config = Arc::new(Config::load());
    let app = telemetry_collector::server::create_app(config);

    // Test GET method
    let get_request = Request::builder()
        .method("GET")
        .uri("/metrics")
        .body(axum::body::Body::empty())
        .unwrap();
    
    let get_response = app.clone().oneshot(get_request).await.unwrap();
    assert_eq!(get_response.status(), 200);

    // Test POST method (should return 405 Method Not Allowed)
    let post_request = Request::builder()
        .method("POST")
        .uri("/metrics")
        .body(axum::body::Body::empty())
        .unwrap();

    let post_response = app.clone().oneshot(post_request).await.unwrap();
    assert_eq!(post_response.status(), 405);
}

#[tokio::test]
async fn test_metrics_response_time() {
    let config = Arc::new(Config::load());
    let app = telemetry_collector::server::create_app(config.clone());

    let request = Request::builder()
        .uri("/metrics")
        .body(axum::body::Body::empty())
        .unwrap();

    // Setting a timeout of 2 seconds for the request
    let response_result = timeout(Duration::from_millis(2000), app.oneshot(request)).await;

    assert!(response_result.is_ok(), "Timeout exceeded for /metrics request");

    let response = response_result.unwrap().unwrap();

    assert_eq!(response.status(), 200);
}