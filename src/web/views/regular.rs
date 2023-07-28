use axum::{
    extract::Form,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::web::forms::PotentialCustomerForm;
use tracing::{event, instrument, Level};

use envy::get_env;

#[instrument]
pub async fn home() -> Response {
    event!(Level::INFO, "inside home!");
    (StatusCode::OK, Html(get_env("DATABASE_URL"))).into_response()
}

pub async fn error_404() -> Response {
    (StatusCode::NOT_FOUND, "nothing to see here").into_response()
}

pub async fn new_potential_customer(Form(input): Form<PotentialCustomerForm>) -> Response {
    input.create().await;
    (StatusCode::OK).into_response()
}
