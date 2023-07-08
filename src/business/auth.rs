use std::sync::Arc;

use crate::web::{requests::RecommendationQueryRequest, responses::non_auth};
use axum::response::Response;

use super::{entities::CustomerInterface, requests::RecommendationRequest, throttling::throttle};

pub async fn auth(payload: RecommendationQueryRequest) -> Response {
    match CustomerInterface::get(Arc::new(String::from("payload.token.clone()"))) {
        Some(customer) => throttle(&customer, RecommendationRequest::new(payload)).await,
        None => non_auth(),
    }
}
