use axum::Router;

use tower_http::services::ServeDir;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

use stefn::AppState;

use crate::{api_docs::ApiDoc, core::dashboard, data, recommendation};

pub fn custom_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest_service("/dist", ServeDir::new("dist"))
        .nest("/dashboard", dashboard::routes(state.clone()))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .nest("/api/:version/", api_routes(state.clone()))
        .with_state(state)
}

fn api_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(recommendation::routes(state.clone()))
        .merge(data::routes(state.clone()))
        .with_state(state)
}
