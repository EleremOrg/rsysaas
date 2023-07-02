use axum::{routing::get, Router};

use super::views::{
    error_404, handle_items, handle_recommendations, handle_users, home, ws_handler,
};

fn api_routes() -> Router {
    Router::new()
        .route("recommendations/", get(handle_recommendations))
        .route("items/", get(handle_items))
        .route("items/:id/", get(handle_items))
        .route("users/", get(handle_users))
        .route("users/:id/", get(handle_users))
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(home))
        .nest("/api/:version/", api_routes())
        .route("ws/", get(ws_handler))
        .fallback(error_404)
}
