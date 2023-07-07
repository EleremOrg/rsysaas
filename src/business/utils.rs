use super::{Customer, RecommendationRequest};
use crate::web::{
    responses::{max_limit, non_auth, not_found, our_fault, success},
    RecommendationQueryRequest,
};
use axum::response::Response;
use orm::errors::CRUDError;

pub async fn auth(payload: RecommendationQueryRequest) -> Response {
    match Customer::get(payload.token.clone()) {
        Some(customer) => throttle(&customer, RecommendationRequest::new(payload)).await,
        None => non_auth(),
    }
}

async fn throttle(customer: &Customer, rec_request: RecommendationRequest) -> Response {
    if can_request(customer).await {
        return get_response(&customer, rec_request).await;
    };
    return max_limit();
}

async fn can_request(_customer: &Customer) -> bool {
    true
}

async fn get_response(customer: &Customer, rec_request: RecommendationRequest) -> Response {
    match customer.get_recommendations(rec_request) {
        Ok(recs) => success(recs),
        Err(err) => match err {
            CRUDError::NotFound => not_found(&rec_request.prod_id),
            CRUDError::MaxRetry => our_fault(),
            _ => our_fault(),
        },
    }
}
