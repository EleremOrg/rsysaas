use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

pub async fn home() -> Response {
    (StatusCode::OK, Html("<h1>Welcome</h1>")).into_response()
}

pub async fn error_404() -> Response {
    (StatusCode::NOT_FOUND, "nothing to see here").into_response()
}
