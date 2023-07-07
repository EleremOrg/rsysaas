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
    delete_entities, error_404, get_entities, get_recommendations, home, list_entities,
    patch_entities, post_entities, put_entities, sse_handler, ws_handler,
};

fn api_routes() -> Router {
    Router::new()
        .route("/recommendations/", get(get_recommendations))
        .route("/entities/", get(list_entities).post(post_entities))
        .route(
            "/entities/:entity/:id/",
            get(get_entities)
                .put(put_entities)
                .patch(patch_entities)
                .delete(delete_entities),
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