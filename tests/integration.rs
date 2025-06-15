use axum::body::Body;
use telemetry_collector::config::Config;

use http::{Request, StatusCode};
use std::sync::Arc;
use tokio::time::{Duration, timeout};
use tower::ServiceExt;

async fn get_metrics_response() -> http::Response<axum::body::Body> {
    let config = mock_config();
    let app = telemetry_collector::server::create_app(config);

    let request = build_request("GET", "/metrics");

    app.oneshot(request).await.unwrap()
}

fn build_request(method: &str, uri: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .body(Body::empty())
        .expect("Failed to build request")
}

fn mock_config() -> Arc<Config> {
    Arc::new(Config {
        port: 8080,
        metrics_path: "/metrics".to_string(),
    })
}

/// Verify that the endpoint returns a 200 status code
#[tokio::test]
async fn test_metrics_endpoint_status_ok() {
    let response = get_metrics_response().await;
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn testing_endpoint_methods() {
    let config = mock_config();
    let app = telemetry_collector::server::create_app(config);

    let get_response = app
        .clone()
        .oneshot(build_request("GET", "/metrics"))
        .await
        .unwrap();
    assert_eq!(get_response.status(), StatusCode::OK);

    // Test POST method (should return 405 Method Not Allowed)
    let post_response = app
        .clone()
        .oneshot(build_request("POST", "/metrics"))
        .await
        .unwrap();
    assert_eq!(post_response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn test_metrics_response_time() {
    let config = mock_config();
    let app = telemetry_collector::server::create_app(config.clone());

    let request = build_request("GET", "/metrics");

    // Setting a timeout of 2 seconds for the request
    let response_result = timeout(Duration::from_millis(2000), app.oneshot(request)).await;

    assert!(
        response_result.is_ok(),
        "Timeout exceeded for /metrics request"
    );

    let response = response_result.unwrap().unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
