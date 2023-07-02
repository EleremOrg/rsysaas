use axum::{routing::get, Router};

use super::views::{
    delete_items, delete_users, error_404, get_items, get_recommendations, get_users, home,
    list_items, list_users, patch_items, patch_users, post_items, post_users, put_items, put_users,
    sse_handler, ws_handler,
};

fn api_routes() -> Router {
    Router::new()
        .route("/recommendations/", get(get_recommendations))
        .route("/items/", get(list_items).post(post_items))
        .route(
            "/items/:id/",
            get(get_items)
                .put(put_items)
                .patch(patch_items)
                .delete(delete_items),
        )
        .route("/users/", get(list_users).post(post_users))
        .route(
            "/users/:id/",
            get(get_users)
                .put(put_users)
                .patch(patch_users)
                .delete(delete_users),
        )
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(home))
        .nest("/api/:version/", api_routes())
        .route("/ws/", get(ws_handler))
        .route("/sse/", get(sse_handler))
        .fallback(error_404)
}
