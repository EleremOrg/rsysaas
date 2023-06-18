use super::models::{RequestModel, ResponseModel};
use crate::data::Customer;
use crate::recsys::RecRequest;
use axum::response::{IntoResponse, Response};
use axum::{extract::Query, http::StatusCode};

pub async fn handle_rest(Query(payload): Query<RequestModel>) -> Response {
    println!("{:?}", payload);
    auth(payload)
}

fn auth(payload: RequestModel) -> Response {
    match Customer::get(payload.token) {
        Some(customer) => throttle(&customer, payload.rec_data),
        None => ResponseModel::non_auth(),
    }
}

fn throttle(customer: &Customer, rec_request: RecRequest) -> Response {
    if can_request(customer) {
        return ResponseModel::success(customer.get_recommendations(rec_request));
    };
    return ResponseModel::max_limit();
}

fn can_request(customer: &Customer) -> bool {
    true
}

pub async fn handle_404() -> Response {
    (StatusCode::NOT_FOUND, "nothing to see here").into_response()
}
