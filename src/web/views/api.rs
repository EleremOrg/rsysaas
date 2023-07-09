use crate::business::interface::CustomerInterface;
use crate::web::{
    requests::RecommendationQueryRequest,
    responses::{non_auth, success},
};
use axum::{extract::Query, http::HeaderMap, response::Response};

pub async fn get_recommendations(
    Query(payload): Query<RecommendationQueryRequest>,
    headers: HeaderMap,
) -> Response {
    match CustomerInterface::is_allowed(payload.entity).await {
        true => success("cools"),
        false => non_auth(),
    }
}
