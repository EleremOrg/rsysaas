use axum::{middleware::from_fn_with_state, Router};

use serde::Deserialize;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

use stefn::{jwt_middleware, AppState};

use super::{api_docs::ApiDoc, ingestion, recommendation, shopify};

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .nest("/api/:version/", api_routes(state.clone()))
        .with_state(state)
}

#[derive(Clone, Deserialize)]
pub struct Tes;
//TODO: remove and fix this struct

fn api_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(ingestion::routes(state.clone()))
        .merge(recommendation::routes(state.clone()))
        .layer(from_fn_with_state(state.clone(), jwt_middleware::<Tes>))
        .nest("/shopify", shopify::routes(state.clone()))
        .with_state(state)
}
