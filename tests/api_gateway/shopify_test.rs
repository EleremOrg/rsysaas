use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;

use serde_json::{json, Value};
use tower::ServiceExt;

use super::common;

#[tokio::test]
async fn test_handle_initial_verification_invalid() {
    let app = common::setup().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/shopify?hmac=700e2dadb827fcc8609e9d5ce208b2e9cdaab9df07390d2cbca10d7c328fc4bf&shop={shop}.myshopify.com&timestamp=1337178173")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_handle_initial_verification_valid() {
    let app = common::setup().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/shopify?hmac=700e2dadb827fcc8609e9d5ce208b2e9cdaab9df07390d2cbca10d7c328fc4bf&shop={shop}.myshopify.com&timestamp=1337178173")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
