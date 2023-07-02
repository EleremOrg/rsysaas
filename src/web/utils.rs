use super::models::{RequestModel, ResponseModel};
use crate::data::Customer;
use crate::recsys::RecRequest;
use axum::response::Response;

pub fn auth(payload: RequestModel) -> Response {
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
            CRUDError::MaxRetry => ResponseModel::our_fault(),
            _ => ResponseModel::our_fault(),
        },
    }
}
