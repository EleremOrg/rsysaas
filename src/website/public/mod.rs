use axum::Router;
use hyper::{
    header::{AUTHORIZATION, CONTENT_TYPE, COOKIE},
    Method,
};
use stefn::WebsiteState;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

mod auth;
mod dashboard;
mod seo;

pub fn routes(state: WebsiteState) -> Router<WebsiteState> {
    Router::new()
        .nest_service("/dist", ServeDir::new("dist"))
        .merge(auth::routes(state.clone()))
        .merge(dashboard::routes(state.clone()))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::POST])
                .allow_headers([CONTENT_TYPE, AUTHORIZATION])
                .allow_origin(Any),
        )
        .with_state(state)
}
