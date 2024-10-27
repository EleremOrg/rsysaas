use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;

use serde_json::{json, Value};
use tower::ServiceExt;

#[tokio::test]
async fn test_404() {
    let app = super::common::setup().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/not-found")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
