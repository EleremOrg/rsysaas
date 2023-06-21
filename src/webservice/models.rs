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
    pub fn not_found(id_not_found: &u32) -> Response {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "message": format!("{id_not_found} not found") })),
        )
            .into_response()
    }
    pub fn our_fault() -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "oupsi"})),
        )
            .into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestModel {
    pub token: String,
    pub user_id: u32,
    pub prod_id: u32,
    pub num_recs: u8,
}

impl RequestModel {
    pub fn rec_data(self) -> RecRequest {
        RecRequest {
            user_id: self.user_id,
            prod_id: self.prod_id,
            num_recs: self.num_recs,
        }
    }
}
