use axum::{body::Body, http::Request};
use http_body_util::BodyExt;

use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_ingest_products() {
    let app = super::common::setup().await;
    let token = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJpbnZhbGlkX3Nob3AuY29tIiwic3ViIjoiMSIsImF1ZCI6ImludmFsaWRfc2hvcC5jb20iLCJleHAiOjE3Mjk5Njk1MzUsImlhdCI6MTcyOTg4MzEzNSwianRpIjoiIiwicm9sZSI6ImFkbWluIn0.Fnf6fTJ1lyDiP4exhRtmEEyFr0ZzGoBhCHVRyuvKPnk";

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/products")
                .header("Content-Type", "application/json")
                .header("Authorization", token)
                .body(Body::from(
                    serde_json::to_vec(&json!(
                        {
                            "category": "Clothing",
                            "products": [
                                {
                                    "id": "1",
                                    "price": 29.99,
                                    "currency": "USD",
                                    "image": "http://example.com/image.png",
                                    "url": "http://example.com",
                                    "description": "A nice shirt",
                                    "specs": {
                                        "category": "Shirt",
                                        "gender": "Unisex",
                                        "size": "M",
                                        "material": "Cotton"
                                    }
                                }
                            ]
                        }
                    ))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = String::from_utf8(body.to_vec()).unwrap();

    println!("{:?}", body);

    // assert_eq!(response.status(), StatusCode::OK);
    // let body = response.into_body().collect().await.unwrap().to_bytes();
    // let body: Value = serde_json::from_slice(&body).unwrap();
    // assert_eq!(body, json!({"results_affected":1}));
}
