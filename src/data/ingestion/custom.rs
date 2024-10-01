use axum::{
    extract::Query,
    http::header::HeaderMap,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{self, OpenApi, ToResponse, ToSchema};

use crate::data::models::{
    AutomotiveCategory, AutomotiveProduct, BooksAndMediaCategory, BooksAndMediaProduct,
    ClothingCategory, ClothingProduct, ClothingType, ElectronicsProduct, ElectronicsSpecs,
    FoodAndBeveragesCategory, FoodAndBeveragesProduct, HealthAndWellnessCategory,
    HealthAndWellnessProduct, HomeGoodsCategory, HomeGoodsProduct, OfficeSuppliesCategory,
    OfficeSuppliesProduct, PersonalCareCategory, PersonalCareProduct, ProductCategory,
    SportsAndOutdoorsCategory, SportsAndOutdoorsProduct, ToysAndGamesCategory, ToysAndGamesProduct,
};

use stefn::{AppError, AppResult, AppState};

#[derive(OpenApi)]
#[openapi(
    paths(handle_products, handle_orders, handle_refunds),
    components(schemas(
        ElectronicsProduct,
        ClothingProduct,
        ProductCategory,
        ElectronicsSpecs,
        ClothingCategory,
        ClothingType,
        HomeGoodsProduct,
        HomeGoodsCategory,
        PersonalCareProduct,
        PersonalCareCategory,
        HealthAndWellnessProduct,
        HealthAndWellnessCategory,
        FoodAndBeveragesProduct,
        FoodAndBeveragesCategory,
        AutomotiveProduct,
        AutomotiveCategory,
        ToysAndGamesProduct,
        ToysAndGamesCategory,
        BooksAndMediaProduct,
        BooksAndMediaCategory,
        SportsAndOutdoorsProduct,
        SportsAndOutdoorsCategory,
        OfficeSuppliesProduct,
        OfficeSuppliesCategory,
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
    created: usize,
    updated: usize,
}

impl ProductsResult {
    fn total_created(num: usize) -> Self {
        Self {
            created: num,
            updated: 0,
        }
    }
    fn total_updated(num: usize) -> Self {
        Self {
            created: 0,
            updated: num,
        }
    }
}

#[utoipa::path(
    post,
    path = "/products",
    request_body = ProductCategory,
    responses(
        (status = 200, body = ProductsResult, description = "Ingest products"),
        (status = "4XX", body = ErrorMessage, description = "Opusi daisy, you messed up"),
        (status = "5XX", body = ErrorMessage, description = "Opusi daisy, we messed up, sorry"),
    )
)]
async fn handle_products(
    state: AppState,
    Json(rec): Json<ProductCategory>,
) -> AppResult<ProductsResult> {
    println!("{rec:?}");
    Ok(Json(ProductsResult::total_created(rec.len())))
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
