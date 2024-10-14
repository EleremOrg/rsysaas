mod core;
mod prestashop;
mod shopify;

use axum::Router;
use utoipa::{self, OpenApi};

use stefn::AppState;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "prestashop", api = prestashop::ApiDoc, tags = ["Prestashop"]),
        (path = "custom", api = core::ApiDoc, tags = ["Custom"]),
    ),
    security(("token_jwt" = []))
)]
pub struct ApiDoc;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/shopify", shopify::routes(state.clone()))
        .nest("/prestashop", prestashop::routes(state.clone()))
        .nest("/custom", core::routes(state.clone()))
        .with_state(state)
}
