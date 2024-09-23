use axum::{
    extract::MatchedPath,
    http::{HeaderValue, Request},
    middleware::from_fn_with_state,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use hyper::{
    header::{AUTHORIZATION, CONTENT_TYPE, COOKIE},
    Method, StatusCode,
};
use std::{
    sync::{atomic::AtomicU64, Arc},
    time::Duration,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    normalize_path::NormalizePathLayer,
    request_id::{MakeRequestId, RequestId},
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    timeout::TimeoutLayer,
    trace::{
        DefaultOnBodyChunk, DefaultOnEos, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse,
        TraceLayer,
    },
    LatencyUnit, ServiceBuilderExt,
};
use tracing::{info_span, Level};

use super::{AppState, Config};

pub fn get_router(config: &Config, state: AppState, routes: Router<AppState>) -> Router<()> {
    let sensitive_headers: Arc<[_]> = vec![AUTHORIZATION, COOKIE].into();
    // Build our middleware stack
    let middleware = ServiceBuilder::new()
        .layer(NormalizePathLayer::trim_trailing_slash())
        // Mark the `Authorization` and `Cookie` headers as sensitive so it doesn't show in logs
        .layer(SetSensitiveRequestHeadersLayer::from_shared(
            sensitive_headers.clone(),
        ))
        .set_x_request_id(MyMakeRequestId::default())
        // Add high level tracing/logging to all requests
        .layer(
            TraceLayer::new_for_http()
                .on_request(|request: &Request<_>, _span: &tracing::Span| {
                    // Log the request with path, method, and any payload
                    tracing::info!(
                        method = %request.method(),
                        path = %request.uri().path(),
                        payload = ?request.body(),
                        "received request"
                    );
                })
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros)
                        .include_headers(true),
                )
                .on_body_chunk(DefaultOnBodyChunk::new())
                .on_eos(DefaultOnEos::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::INFO)),
        )
        .sensitive_response_headers(sensitive_headers)
        // Set a timeout
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        // Compress responses
        .compression()
        .propagate_x_request_id()
        // Set a `Content-Type` if there isn't one already.
        .insert_response_header_if_not_present(
            CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        )
        .layer(std_cors(config));

    Router::new()
        .nest("/", routes)
        .fallback(error_404)
        .layer(middleware)
        .with_state(state)
}

#[derive(Clone, Default)]
struct MyMakeRequestId {
    counter: Arc<AtomicU64>,
}

use std::sync::atomic::Ordering;

impl MakeRequestId for MyMakeRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        self.counter
            .fetch_add(1, Ordering::SeqCst)
            .to_string()
            .parse()
            .ok()
            .map(|r| RequestId::new(r))
    }
}

fn std_cors(config: &Config) -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_origin(Any)
}

async fn error_404() -> Response {
    (StatusCode::NOT_FOUND, Html("<h1>Nothing to see here</h1>")).into_response()
}
