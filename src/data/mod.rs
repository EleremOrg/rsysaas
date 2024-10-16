mod prestashop;

use axum::Router;
use utoipa::{self, OpenApi};

use stefn::AppState;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "prestashop", api = prestashop::ApiDoc, tags = ["Prestashop"]),
    ),
    security(("token_jwt" = []))
)]
pub struct ApiDoc;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/prestashop", prestashop::routes(state.clone()))
        .with_state(state)
}
