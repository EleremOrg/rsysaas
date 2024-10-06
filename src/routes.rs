use axum::{middleware::from_fn_with_state, routing::get, Router};

use tower_http::services::ServeDir;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

use stefn::{jwt_middleware, AppState};

use crate::{api_docs::ApiDoc, dashboard::hello, data, recommendation};

pub fn custom_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(hello))
        .nest_service("/dist", ServeDir::new("dist"))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .nest("/api/:version/", api_routes(state.clone()))
        .with_state(state)
}

fn api_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(recommendation::routes(state.clone()))
        .merge(data::routes(state.clone()))
        //.layer(from_fn_with_state(state.clone(), jwt_middleware))
        .with_state(state)
}
