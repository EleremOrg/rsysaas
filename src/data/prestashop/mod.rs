use axum::{
    extract::Query,
    http::header::HeaderMap,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{self, OpenApi, ToResponse, ToSchema};

use stefn::{AppError, AppResult, AppState};

#[derive(OpenApi)]
#[openapi(
    paths(handle_products, handle_orders, handle_refunds),
    components(schemas(Product, Order, Refund),
    responses()),
    security(("token_jwt" = [])),
    tags((name = "Prestashop"))
)]
pub struct ApiDoc;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/products", post(handle_products))
        .route("/orders", post(handle_orders))
        .route("/refunds", post(handle_refunds))
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct Product {
    id: u64,
    name: String,
}

#[utoipa::path(
    post,
    path = "/products",
    request_body = Product,
    responses(
        (status = 200, body = usize, description = "Ingest products"),
        (status = "4XX", body = ErrorMessage, description = "Opusi daisy, you messed up"),
        (status = "5XX", body = ErrorMessage, description = "Opusi daisy, we messed up, sorry"),
    )
)]
async fn handle_products(state: AppState, Json(rec): Json<Product>) -> AppResult<usize> {
    println!("{rec:?}");
    Ok(Json(200))
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct Order {
    id: u64,
    product_id: u64,
}

#[utoipa::path(
    post,
    path = "/orders",
    request_body = Order,
    responses(
        (status = 200, body = usize, description = "Ingest orders"),
        (status = "4XX", body = ErrorMessage, description = "Opusi daisy, you messed up"),
        (status = "5XX", body = ErrorMessage, description = "Opusi daisy, we messed up, sorry"),
    )
)]
async fn handle_orders(state: AppState, Json(rec): Json<Order>) -> AppResult<usize> {
    println!("{rec:?}");
    Ok(Json(200))
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct Refund {
    id: u64,
    order_id: u64,
    product_id: u64,
}

#[utoipa::path(
    post,
    path = "/refunds",
    request_body = Refund,
    responses(
        (status = 200, body = usize, description = "Ingest refunds"),
        (status = "4XX", body = ErrorMessage, description = "Opusi daisy, you messed up"),
        (status = "5XX", body = ErrorMessage, description = "Opusi daisy, we messed up, sorry"),
    )
)]
async fn handle_refunds(state: AppState, Json(rec): Json<Refund>) -> AppResult<usize> {
    println!("{rec:?}");
    Ok(Json(200))
}
