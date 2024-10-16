use axum::Router;

use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

use stefn::AppState;

use super::{api_docs::ApiDoc, integration, recommendation, shopify};

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .nest("/api/:version/", api_routes(state.clone()))
        .with_state(state)
}

fn api_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/shopify", shopify::routes(state.clone()))
        .merge(integration::routes(state.clone()))
        .merge(recommendation::routes(state.clone()))
        .with_state(state)
}
