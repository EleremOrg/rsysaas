use axum::{
    extract::Query,
    http::header::HeaderMap,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{self, OpenApi, ToResponse, ToSchema};

use crate::{
    server::{AppError, AppResult},
    AppState,
};

use super::{custom, prestashop, shopify};

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "shopify", api = shopify::ApiDoc, tags = ["Shopify"]),
        (path = "prestashop", api = prestashop::ApiDoc, tags = ["Prestashop"]),
        (path = "custom", api = custom::ApiDoc, tags = ["Custom"]),
    ),
    security(("token_jwt" = []))
)]
pub struct ApiDoc;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/shopify", shopify::routes(state.clone()))
        .nest("/prestashop", prestashop::routes(state.clone()))
        .nest("/custom", custom::routes(state.clone()))
        .with_state(state)
}
