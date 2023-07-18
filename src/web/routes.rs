use super::{
    middlewares::cors,
    views::{
        api::{get_embed_recommendations, get_recommendations},
        regular::{error_404, home},
        sse::sse_handler,
        ws::ws_handler,
    },
};
use crate::data::models::{
    customer::Customer,
    invfin::{association::Association, company::Company, term::Term},
    user::User,
};
use crate::web::facade::View;
use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::Response,
    routing::get,
    Router,
};
use hyper::{http, Method};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    classify::ServerErrorsFailureClass,
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing::{info_span, Span};

fn api_routes() -> Router {
    Router::new()
        .merge(User::routes())
        .merge(Term::routes())
        .merge(Company::routes())
        .merge(Association::routes())
        .merge(Customer::routes())
        .merge(recommendations_routes())
}

fn recommendations_routes() -> Router {
    Router::new()
        .route("/ws/", get(ws_handler))
        .route("/sse/", get(sse_handler))
        .route("/recommendations/", get(get_recommendations))
        .route("/embed-recommendations/", get(get_embed_recommendations))
        .layer(cors())
}

pub fn routes() -> Router {
    Router::new()
        .nest_service(
            "/assets/embed-widget.js",
            ServeDir::new("assets/embed-widget.js"),
        )
        .route("/", get(home))
        .nest("/api/:version/", api_routes())
        .fallback(error_404)
        // `TraceLayer` is provided by tower-http so you have to add that as a dependency.
        // It provides good defaults but is also very customizable.
        //
        // See https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html for more details.
        //
        // If you want to customize the behavior using closures here is how.
        .layer(
            ServiceBuilder::new().layer(
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
                        |_trailers: Option<&HeaderMap>,
                         _stream_duration: Duration,
                         _span: &Span| {
                            // ...
                        },
                    )
                    .on_failure(
                        |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                            // ...
                        },
                    ),
            ),
        )
}
