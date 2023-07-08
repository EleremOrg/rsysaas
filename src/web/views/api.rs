use crate::business::auth::auth;
use crate::web::requests::RecommendationQueryRequest;
use axum::{extract::Query, response::Response};

pub async fn get_recommendations(Query(payload): Query<RecommendationQueryRequest>) -> Response {
    auth(payload).await
}
