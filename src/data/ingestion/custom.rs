use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, Sqlite};
use utoipa::{self, OpenApi, ToSchema};

use crate::data::models::{
    complements::{Order, Refund},
    products::{
        AutomotiveCategory, AutomotiveProduct, BooksAndMediaCategory, BooksAndMediaProduct,
        ClothingCategory, ClothingProduct, ClothingType, ElectronicsProduct, ElectronicsSpecs,
        FoodAndBeveragesCategory, FoodAndBeveragesProduct, HealthAndWellnessCategory,
        HealthAndWellnessProduct, HomeGoodsCategory, HomeGoodsProduct, OfficeSuppliesCategory,
        OfficeSuppliesProduct, PersonalCareCategory, PersonalCareProduct, ProductCategory,
        SportsAndOutdoorsCategory, SportsAndOutdoorsProduct, ToysAndGamesCategory,
        ToysAndGamesProduct,
    },
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
    fn total_updated(num: u64) -> Self {
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
    Json(payload): Json<ProductCategory>,
) -> AppResult<ProductsResult> {
    let mut tx = state
        .primary_database
        .begin()
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?;
    let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO products(id, company_pk, meta) ");

    // Note that `.into_iter()` wasn't needed here since `users` is already an iterator.
    // query_builder.push_values(payload, |mut b, user| {
    //     // If you wanted to bind these by-reference instead of by-value,
    //     // you'd need an iterator that yields references that live as long as `query_builder`,
    //     // e.g. collect it to a `Vec` first.
    //     b.push_bind(user.id)
    //         .push_bind(user.username)
    //         .push_bind(user.email)
    //         .push_bind(user.password);
    // });

    let result = query_builder
        .build()
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?
        .rows_affected();

    let _ = tx
        .commit()
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?;
    Ok(Json(ProductsResult::total_created(result)))
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
