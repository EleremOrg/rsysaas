use super::models::{RequestModel, ResponseModel};
use crate::data::Customer;
use crate::recsys::RecRequest;
use axum::response::{IntoResponse, Json, Response};
use axum::{extract::Query, http::StatusCode};
use serde_json::json;

pub async fn handle_rest(Query(payload): Query<RequestModel>) -> Response {
    println!("{:?}", payload);

    auth(payload)
}

fn auth(payload: RequestModel) -> Response {
    match Customer::get(payload.token) {
        Some(customer) => throttle(customer, payload.rec_data),
        None => (StatusCode::FORBIDDEN, Json(json!({"message": "Not auth"}))).into_response(),
    }
}

fn throttle(customer: Customer, rec_request: RecRequest) -> Response {
    if can_request(customer) {
        let response = ResponseModel {
            recs: customer.get_recommendations(rec_request),
        };
        return (StatusCode::OK, Json(response)).into_response();
    };
    return (
        StatusCode::NOT_ACCEPTABLE,
        Json(json!({"message": "limit exceeded"})),
    )
        .into_response();
}

fn can_request(customer: Customer) -> bool {
    true
}

pub async fn handle_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
