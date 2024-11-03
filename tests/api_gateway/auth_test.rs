use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;

use serde_json::{json, Value};
use tower::ServiceExt;

#[tokio::test]
async fn test_get_token() {
    let app = super::common::setup().await;

    let response = app
        .oneshot(
            Request::builder()
            .method("GET")
            .uri("/api/v1/auth/token")
            .header("Authorization", "Basic dXNlcjFAZXhhbXBsZS5jb206cGFzc3dvcmQxMjM")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
