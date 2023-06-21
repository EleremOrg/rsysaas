use super::models::{RequestModel, ResponseModel};
use crate::data::{CRUDError, Customer};
use crate::recsys::RecRequest;
use axum::response::{IntoResponse, Response};
use axum::{extract::Query, http::StatusCode};

pub async fn handle_rest(Query(payload): Query<RequestModel>) -> Response {
    auth(payload)
}

fn auth(payload: RequestModel) -> Response {
    match Customer::get(payload.token.clone()) {
        Some(customer) => throttle(&customer, payload.rec_data()),
        None => ResponseModel::non_auth(),
    }
}

fn throttle(customer: &Customer, rec_request: RecRequest) -> Response {
    if can_request(customer) {
        return get_response(&customer, rec_request);
    };
    return ResponseModel::max_limit();
}

fn can_request(customer: &Customer) -> bool {
    true
}

fn get_response(customer: &Customer, rec_request: RecRequest) -> Response {
    match customer.get_recommendations(rec_request) {
        Ok(recs) => ResponseModel::success(recs),
        Err(err) => match err {
            CRUDError::NotFound => ResponseModel::not_found(&rec_request.prod_id),
            CRUDError::MaxRetryError => ResponseModel::our_fault(),
        },
    }
}

pub async fn handle_404() -> Response {
    (StatusCode::NOT_FOUND, "nothing to see here").into_response()
}
