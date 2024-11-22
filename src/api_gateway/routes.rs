use axum::{middleware::from_fn_with_state, Router};
use hyper::{
    header::{AUTHORIZATION, CONTENT_TYPE, COOKIE},
    Method,
};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

use stefn::{jwt_middleware, APIState};

use super::{
    api_docs::ApiDoc,
    auth::{self, PrivateClaims},
    ingestion, recommendation, shopify,
};

pub fn routes(state: APIState) -> Router<APIState> {
    Router::new()
        .merge(RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .nest("/api/:version", api_routes(state.clone()))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::POST])
                .allow_headers([CONTENT_TYPE, AUTHORIZATION])
                .allow_origin(Any),
        )
}

fn api_routes(state: APIState) -> Router<APIState> {
    Router::new()
        .merge(ingestion::routes(state.clone()))
        .nest("/recommendations", recommendation::routes(state.clone()))
        .layer(from_fn_with_state(
            state.clone(),
            jwt_middleware::<PrivateClaims>,
        ))
        .nest("/auth", auth::routes(state.clone()))
        .nest("/shopify", shopify::routes(state.clone()))
}
