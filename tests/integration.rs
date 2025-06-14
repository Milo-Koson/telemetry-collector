use tower::ServiceExt;
use http::Request;

async fn get_metrics_response() -> http::Response<axum::body::Body> {
    let app = telemetry_collector::server::create_app();

    let request = Request::builder()
        .uri("/metrics")
        .body(axum::body::Body::empty())
        .unwrap();

    app.oneshot(request).await.unwrap()
}

#[tokio::test]
async fn test_metrics_endpoint_status_ok() {
    let response = get_metrics_response().await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_metrics_endpoint_response_json() {
    let response = get_metrics_response().await;
    assert_eq!(response.status(), 200);

    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice::<telemetry_collector::types::TelemetryReport>(&body).unwrap();
}