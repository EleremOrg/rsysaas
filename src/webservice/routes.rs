use axum::{routing::get, Router};

use super::rest::{handle_404, handle_rest};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(handle_rest))
        .fallback(handle_404)
}
