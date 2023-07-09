use crate::business::{interface::CustomerInterface, requests::RecommendationRequest};
use crate::data::errors::CRUDError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;
use serde_json::json;
use std::fmt::Debug;

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
pub fn not_found<T: Debug>(data: &T) -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "message": format!("{:#?} not found", data) })),
    )
        .into_response()
}
pub fn wrong_query<T: Debug>(query: &T) -> Response {
    (
        StatusCode::NOT_ACCEPTABLE,
        Json(json!({ "message": format!("wrong query {:#?}", query) })),
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

pub async fn match_error<T: Serialize + Send, P: Serialize + Send + Debug>(
    result: Result<T, CRUDError>,
    params: &P,
) -> Response {
    match result {
        Ok(u) => success(u),
        Err(err) => match err {
            CRUDError::NotFound => not_found(params),
            CRUDError::MaxRetry => max_limit(),
            CRUDError::WrongParameters => not_found(params),
            CRUDError::Write => our_fault(),
            CRUDError::Delete => our_fault(),
            CRUDError::JsonError => our_fault(),
        },
    }
}

// TODO: fix
pub async fn get_response(
    customer: &CustomerInterface,
    rec_request: RecommendationRequest,
) -> Response {
    // match customer.get_recommendations(rec_request) {
    //     Ok(recs) => success(recs),
    //     Err(err) => match err {
    //         CRUDError::NotFound => not_found(&rec_request.prod_id),
    //         CRUDError::MaxRetry => our_fault(),
    //         _ => our_fault(),
    //     },
    // }
    success("hey")
}
