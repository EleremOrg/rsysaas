mod auth;
mod webhooks;

use auth::{handle_authentication, handle_initial_verification};

use axum::{
    routing::{get, post},
    Router,
};
use stefn::AppState;
use webhooks::{handle_app, handle_bulk_operations, handle_products};

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(handle_initial_verification))
        .route("/auth/callback", get(handle_authentication))
        .route("/app/uninstalled", post(handle_app))
        .route("/products", post(handle_products))
        .route("/handle_bulk_operations", post(handle_bulk_operations))
        .with_state(state)
}
