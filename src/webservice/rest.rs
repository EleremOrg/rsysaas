use crate::recsys::Recommendation;

use super::models::{RestRequest, RestResponse};
use axum::response::{IntoResponse, Json, Response};
use axum::{extract::Query, http::StatusCode};

pub async fn handle_rest(Query(payload): Query<RestRequest>) -> Response {
    println!("{:?}", payload);

    let response = RestResponse {
        recs: Recommendation::generate_recommendations(payload.num_recs),
        message: "All good".to_string(),
    };
    (StatusCode::OK, Json(response)).into_response()
}

pub async fn handle_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
