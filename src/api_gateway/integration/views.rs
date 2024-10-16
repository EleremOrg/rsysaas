use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::{self, OpenApi, ToSchema};

use super::{
    controllers::run_transaction,
    models::{
        BooksAndMediaCategory, BooksAndMediaProduct, ClothingCategory, ClothingGender,
        ClothingProduct, Order, ProductCategory, Refund, SportsAndOutdoorsCategory,
        SportsAndOutdoorsProduct,
    },
};

use stefn::{AppError, AppResult, AppState, ErrorMessage};

#[derive(OpenApi)]
#[openapi(
    paths(handle_products, handle_orders, handle_refunds),
    components(schemas(
        ProductCategory,
        ClothingProduct,
        ClothingCategory,
        ClothingGender,
        BooksAndMediaProduct,
        BooksAndMediaCategory,
        SportsAndOutdoorsProduct,
        SportsAndOutdoorsCategory,
        Order,
        Refund
    ),
    responses()),
    security(("token_jwt" = [])),
    tags((name = "Custom"))
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
struct ProductsResult {
    created: u64,
    updated: u64,
}

impl ProductsResult {
    fn total_created(num: u64) -> Self {
        Self {
            created: num,
            updated: 0,
        }
    }
}

#[utoipa::path(
    post,
    path = "products",
    request_body = ProductCategory,
    responses(
        (status = 200, body = ProductsResult, description = "Ingest products"),
        (status = "4XX", body = ErrorMessage, description = "Opusi daisy, you messed up"),
        (status = "5XX", body = ErrorMessage, description = "Opusi daisy, we messed up, sorry"),
    )
)]
async fn handle_products(
    state: AppState,
    Json(payload): Json<ProductCategory>,
) -> AppResult<ProductsResult> {
    let result = run_transaction(state, payload).await?;
    Ok(Json(ProductsResult::total_created(result)))
}

#[utoipa::path(
    post,
    path = "orders",
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

#[utoipa::path(
    post,
    path = "refunds",
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
