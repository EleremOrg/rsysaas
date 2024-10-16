use axum::{routing::get, Router};

use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

use stefn::AppState;

use super::api_docs::ApiDoc;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .nest("/api/:version/", api_routes(state.clone()))
        .with_state(state)
}

fn api_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("test", get(|| async { "hello" }))
        .with_state(state)
}
