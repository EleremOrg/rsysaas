use std::fmt::Debug;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use serde_json::json;

pub fn success<T: serde::Serialize>(data: T) -> Response {
    (StatusCode::OK, Json(json!({ "data": data }))).into_response()
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
pub fn not_found(id: &u32) -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "message": format!("{id} not found") })),
    )
        .into_response()
}
pub fn wrong_query<T: Debug>(query: &T) -> Response {
    (
        StatusCode::NOT_ACCEPTABLE,
        Json(json!({ "message": format!("wrong query {:?}", query) })),
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
