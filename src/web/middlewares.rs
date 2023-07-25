use hyper::{http, Method};

use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::set_header::MakeHeaderValue;

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_origin(Any)
}

pub fn post_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_origin(Any)
}
