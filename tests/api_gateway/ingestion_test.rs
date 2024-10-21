use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;

use serde_json::{json, Value};
use tower::ServiceExt;

#[tokio::test]
async fn test_ingest_products() {
    let app = super::common::setup();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/products")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!(
                        {
                            "products": [
                              {
                                "category": "Shirt",
                                "gender": "Men",
                                "id": "12",
                                "image": "",
                                "price": 1,
                                "url": ""
                              }
                            ],
                            "target": "Clothing"
                          }
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
    assert_eq!(body, json!({"created":1,"updated":0}));
}
