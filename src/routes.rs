use axum::{middleware::from_fn_with_state, Router};

use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

use crate::{
    api_docs::ApiDoc,
    data, recommendation,
    server::{jwt_middleware, AppState},
};

pub fn custom_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .nest("/api/:version/", api_routes(state.clone()))
        .with_state(state)
}

fn api_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(recommendation::routes(state.clone()))
        .nest("/data", data::routes(state.clone()))
        //.layer(from_fn_with_state(state.clone(), jwt_middleware))
        .with_state(state)
}