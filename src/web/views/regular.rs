use axum::{
    extract::Form,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
};

use crate::{
    business::requests::handle_recommendation_redirection,
    web::{forms::PotentialCustomerForm, requests::recommendation::RecommendationRedirect},
};

pub async fn home() -> Response {
    (StatusCode::OK, Html("<h1>Welcome to Elerem</h1>")).into_response()
}

pub async fn error_404() -> Response {
    (StatusCode::NOT_FOUND, Html("<h1>Nothing to see here</h1>")).into_response()
}

pub async fn new_potential_customer(Form(input): Form<PotentialCustomerForm>) -> Response {
    input.create().await;
    (StatusCode::OK).into_response()
}

pub async fn redirect_recommendation(payload: RecommendationRedirect) -> Redirect {
    let new_url = handle_recommendation_redirection(&payload).await;
    Redirect::to(&format!("https://{}/", new_url))
}
