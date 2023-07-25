use axum::{
    extract::Form,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::web::forms::PotentialCustomerForm;

pub async fn home() -> Response {
    (StatusCode::OK, Html("<h1>Welcome</h1>")).into_response()
}

pub async fn error_404() -> Response {
    (StatusCode::NOT_FOUND, "nothing to see here").into_response()
}

pub async fn new_potential_customer(Form(input): Form<PotentialCustomerForm>) -> Response {
    input.create().await;
    (StatusCode::OK).into_response()
}
