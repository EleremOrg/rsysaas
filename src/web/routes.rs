use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::Response,
    routing::get,
    Router,
};
use std::time::Duration;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};

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
        // `TraceLayer` is provided by tower-http so you have to add that as a dependency.
        // It provides good defaults but is also very customizable.
        //
        // See https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html for more details.
        //
        // If you want to customize the behavior using closures here is how.
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        // ...
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        // ...
                    },
                ),
        )
}
