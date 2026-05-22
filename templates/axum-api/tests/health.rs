use axum::{
    body::Body,
    routing::get,
    Router,
};
use tower::ServiceExt;
use http::{StatusCode, header::{CONTENT_TYPE, HeaderValue}};

#[tokio::test]
async fn healthz_returns_ok() {
    let app = Router::new()
        .route("/healthz", get(|| async { "ok" }));

    let response = app
        .oneshot(
            http::Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), 262144)
        .await
        .unwrap();
    assert_eq!(&body[..], b"ok");
}

#[tokio::test]
async fn readyz_returns_ready() {
    let app = Router::new()
        .route("/readyz", get(|| async { "ready" }));

    let response = app
        .oneshot(
            http::Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), 262144)
        .await
        .unwrap();
    assert_eq!(&body[..], b"ready");
}

#[tokio::test]
async fn version_returns_json() {
    use serde_json::Value;

    let app = Router::new()
        .route("/version", get(|| async {
            axum::Json(serde_json::json!({
                "version": env!("CARGO_PKG_VERSION")
            }))
        }));

    let response = app
        .oneshot(
            http::Request::builder()
                .uri("/version")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(CONTENT_TYPE).unwrap(),
        HeaderValue::from_static("application/json")
    );

    let body = axum::body::to_bytes(response.into_body(), 262144)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert!(json.get("version").is_some());
}