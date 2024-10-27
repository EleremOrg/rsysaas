use axum::{middleware::from_fn_with_state, Router};

use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

use stefn::{jwt_middleware, AppState};

use super::{
    api_docs::ApiDoc,
    auth::{self, PrivateClaims},
    ingestion, recommendation, shopify,
};

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .nest("/api/:version", api_routes(state.clone()))
        .with_state(state)
}

fn api_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(ingestion::routes(state.clone()))
        .merge(recommendation::routes(state.clone()))
        .layer(from_fn_with_state(
            state.clone(),
            jwt_middleware::<PrivateClaims>,
        ))
        .nest("/auth", auth::routes(state.clone()))
        .nest("/shopify", shopify::routes(state.clone()))
        .with_state(state)
}
