use axum::{
    extract::Query,
    http::header::HeaderMap,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{self, OpenApi, ToResponse, ToSchema};

use crate::{
    server::{AppError, AppResult},
    AppState,
};

#[derive(OpenApi)]
#[openapi(
    paths(handle_products),
    components(schemas(),
    responses()),
    security(("token_jwt" = [])),
    tags((name = "Shopify"))
)]
pub struct ApiDoc;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/products", post(handle_products))
        .with_state(state)
}

#[derive(Debug, Deserialize)]
struct ShopifyQueryParams {
    embedded: Option<String>, // optional fields
    hmac: String,
    host: String,
    id_token: Option<String>, // optional JWT token
    locale: Option<String>,   // optional locale
    session: Option<String>,  // optional session ID
    shop: String,
    timestamp: u64, // timestamp as an integer
}

#[utoipa::path(
    post,
    path = "/products",
    request_body = RecommendationRequest,
    responses(
        (status = 200, body = Vec<Recommendation>, description = "Recommendations for a client"),
        (status = "4XX", body = ErrorMessage, description = "Opusi daisy, you messed up"),
        (status = "5XX", body = ErrorMessage, description = "Opusi daisy, we messed up, sorry"),
    )
)]
async fn handle_products(
    headers: HeaderMap,
    state: AppState,
    Json(rec): Json<Value>,
) -> AppResult<Vec<Value>> {
    println!("{rec:?}");
    println!("{headers:?}");
    Ok(Json(vec![]))
}
