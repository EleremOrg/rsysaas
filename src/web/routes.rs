use axum::{
    extract::MatchedPath,
    http::Request,
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    services::ServeDir,
    trace::{
        DefaultOnBodyChunk, DefaultOnEos, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse,
        TraceLayer,
    },
    LatencyUnit,
};
use tracing::{info_span, Level};

use super::{
    middlewares::{cors, post_cors},
    views::{
        api::{get_embed_recommendations, get_recommendations},
        regular::{error_404, home, new_potential_customer},
        sse::sse_handler,
        ws::ws_handler,
    },
};
use crate::data::models::{
    customer::Customer,
    invfin::{association::Association, company::Company, term::Term},
    user::User,
};
use crate::web::interface::View;

pub fn routes() -> Router {
    Router::new()
        .route("/save-new-user/", post(new_potential_customer))
        .layer(post_cors())
        .route("/", get(home))
        .nest_service(
            "/assets/embed-widget.js",
            ServeDir::new("assets/embed-widget.js"),
        )
        .nest("/api/:version/", api_routes())
        .fallback(error_404)
        // `TraceLayer` is provided by tower-http so you have to add that as a dependency.
        // It provides good defaults but is also very customizable.
        //
        // See https://docs.rs/tower-http/latest/tower_http/trace/index.html for more details.
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
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .latency_unit(LatencyUnit::Micros)
                            .include_headers(true),
                    )
                    .on_body_chunk(DefaultOnBodyChunk::new())
                    .on_eos(DefaultOnEos::new().level(Level::INFO))
                    .on_failure(DefaultOnFailure::new().level(Level::INFO)),
            ),
        )
}

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
