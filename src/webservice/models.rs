use crate::recsys::models::{RecRequest, Recommendation};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseModel {
    pub recs: Vec<Recommendation>,
}

impl ResponseModel {
    pub fn success(recs: Vec<Recommendation>) -> Response {
        (StatusCode::OK, Json(json!({ "recs": recs }))).into_response()
    }
    pub fn non_auth() -> Response {
        (StatusCode::FORBIDDEN, Json(json!({"message": "Not auth"}))).into_response()
    }
    pub fn max_limit() -> Response {
        (
            StatusCode::NOT_ACCEPTABLE,
            Json(json!({"message": "limit exceeded"})),
        )
            .into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestModel {
    pub token: String,
    pub rec_data: RecRequest,
}
