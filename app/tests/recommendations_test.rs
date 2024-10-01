mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;

use serde_json::{json, Value};
use tower::ServiceExt; // for `call`, `oneshot`, and `ready` // for `collect`

#[tokio::test]
async fn test_404() {
    let app = common::setup();

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

#[tokio::test]
async fn test_wrong_media_type() {
    let app = common::setup();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/recommendations")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
}

#[tokio::test]
async fn test_bad_request() {
    let app = common::setup();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/recommendations")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!([1, 2, 3, 4])).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_recommendation() {
    let app = common::setup();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/recommendations")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!(
                        {"number_recommendations":3,
                    "target":"User"}
                    ))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        body,
        json!([
            {
                "id": 1,
                "score": 1.1,
                "url": "String",
                "image": "String",
                "title": "String",
                "resume": "String",
            }
        ])
    );
}
