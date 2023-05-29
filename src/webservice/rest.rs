use super::models::{RestRequest, RestResponse};
use axum::response::{IntoResponse, Json, Response};
use axum::{extract::Query, http::StatusCode};
pub async fn handle_rest(Query(payload): Query<RestRequest>) -> Response {
    println!("{:?}", payload);

    let response = RestResponse {
        message: "Hello, World!".to_string(),
    };
    (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
    //Json(response)
}
pub async fn handle_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
