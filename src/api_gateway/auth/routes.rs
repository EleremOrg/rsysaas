use axum::{extract::State, routing::post, Json, Router};
use axum_extra::{
    headers::{authorization::Basic, Authorization},
    TypedHeader,
};
use stefn::{APIState, AppResult, ErrorMessage};
use utoipa::{self, OpenApi};

use super::{dtos::JWTResponse, services::get_token};

#[derive(OpenApi)]
#[openapi(
    paths(handle_get_token),
    components(schemas(JWTResponse), responses(JWTResponse)),
    security(),
    tags()
)]
pub struct ApiDoc;

pub fn routes(state: APIState) -> Router<APIState> {
    Router::new()
        .route("/token", post(handle_get_token))
        .with_state(state)
}

#[utoipa::path(
    post,
    path = "token",
    responses(
        (status = 200, body = JWTResponse, description = "Login to get a token"),
        (status = "4XX", body = ErrorMessage, description = "Opusi daisy, you messed up"),
        (status = "5XX", body = ErrorMessage, description = "Opusi daisy, we messed up, sorry"),
    )
)]
async fn handle_get_token(
    state: State<APIState>,
    // TypedHeader(basic): TypedHeader<Authorization<Basic>>,
) -> AppResult<JWTResponse> {
    get_token(
        state.database(),
        state.encoding(),
        "basic.username()",
        "basic.password()",
    )
    .await
    .map(|t| Json(JWTResponse::new(t)))
}
