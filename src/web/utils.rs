use super::models::RequestModel;
use super::responses::{max_limit, non_auth, not_found, our_fault, success};
use crate::data::{CRUDError, Customer};
use crate::recsys::RecRequest;
use axum::response::Response;

pub async fn auth(payload: RequestModel) -> Response {
    match Customer::get(payload.token.clone()) {
        Some(customer) => throttle(&customer, payload.rec_data()).await,
        None => non_auth(),
    }
}

async fn throttle(customer: &Customer, rec_request: RecRequest) -> Response {
    if can_request(customer).await {
        return get_response(&customer, rec_request).await;
    };
    return max_limit();
}

async fn can_request(customer: &Customer) -> bool {
    true
}

async fn get_response(customer: &Customer, rec_request: RecRequest) -> Response {
    match customer.get_recommendations(rec_request) {
        Ok(recs) => success(recs),
        Err(err) => match err {
            CRUDError::NotFound => not_found(&rec_request.prod_id),
            CRUDError::MaxRetry => our_fault(),
            _ => our_fault(),
        },
    }
}
