use std::sync::Arc;

use crate::business::recommendations::Recommendation;
use crate::business::{auth::get_bearer_token, interface::CustomerInterface};
use crate::data::errors::CRUDError;
use crate::web::{
    requests::{EmbedRecommendationQueryRequest, RecommendationQueryRequest},
    responses::{non_auth, our_fault, success, wrong_query},
};
use axum::{extract::Query, http::HeaderMap, response::Response};

pub async fn get_recommendations(
    Query(payload): Query<RecommendationQueryRequest>,
    headers: HeaderMap,
) -> Response {
    let token = match get_bearer_token(&headers).await {
        Some(token) => token,
        None => return non_auth(),
    };
    let customer = match CustomerInterface::get(token).await {
        Ok(customer) => customer,
        Err(_) => return non_auth(),
    };
    if customer.models_related.contains(payload.entity.as_ref()) {
        return match customer.get_recommendations(&payload).await {
            Ok(recs) => success(recs),
            Err(err) => match err {
                CRUDError::NotFound => wrong_query(&payload),
                CRUDError::MaxRetry => our_fault(),
                _ => our_fault(),
            },
        };
    }
    wrong_query(&payload)
}

pub async fn get_embed_recommendations(
    Query(payload): Query<EmbedRecommendationQueryRequest>,
) -> Response {
    success([
        Recommendation::new(1, 0.98, Arc::new(String::from("invfin"))),
        Recommendation::new(2, 0.88, Arc::new(String::from("invfin"))),
        Recommendation::new(3, 0.78, Arc::new(String::from("invfin"))),
        Recommendation::new(4, 0.68, Arc::new(String::from("invfin"))),
    ])
}
