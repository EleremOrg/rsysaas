use axum::{
    extract::Form,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
};
use tracing::{event, instrument, Level};

use envy::get_env;

use crate::{
    business::requests::handle_recommendation_redirection,
    web::{forms::PotentialCustomerForm, requests::recommendation::RecommendationRedirect},
};

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

pub async fn redirect_recommendation(payload: RecommendationRedirect) -> Redirect {
    let new_url = handle_recommendation_redirection(&payload).await;
    Redirect::to(&format!("http://{}/", new_url))
}
