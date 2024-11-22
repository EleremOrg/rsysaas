use axum::{middleware::from_fn_with_state, Router};
use hyper::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use stefn::{login_required_middleware, sessions_middleware, WebsiteState};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

mod auth;
mod dashboard;
mod seo;

pub fn routes(state: WebsiteState) -> Router<WebsiteState> {
    Router::new()
        .merge(dashboard::routes(state.clone()))
        .layer(from_fn_with_state(state.clone(), login_required_middleware))
        .merge(auth::routes(state.clone()))
        .layer(from_fn_with_state(state.clone(), sessions_middleware))
        .nest_service("/dist", ServeDir::new("dist"))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::POST])
                .allow_headers([CONTENT_TYPE, AUTHORIZATION])
                .allow_origin(Any),
        )
        .with_state(state)
}
