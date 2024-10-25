use std::env::current_exe;

use axum::{routing::post, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::{self, OpenApi, ToSchema};

use crate::{
    api_gateway::routes::Tes,
    entities::{
        events::{Command, Source},
        products::{
            BooksAndMediaCategory, BooksAndMediaProduct, ClothingCategory, ClothingGender,
            ClothingProduct, Order, ProductCategory, Refund, SportsAndOutdoorsCategory,
            SportsAndOutdoorsProduct,
        },
    },
};

use stefn::{
    AppResult, AppState, Broker, ErrorMessage, EventFactory, EventMetadata, JWTUserRequest,
};

use super::applications::send_events;

#[derive(OpenApi)]
#[openapi(
    paths(handle_insert_products, handle_upsert_products, handle_orders, handle_refunds),
    components(schemas(
        ProductCategory,
        ClothingProduct,
        ClothingCategory,
        ClothingGender,
        BooksAndMediaProduct,
        BooksAndMediaCategory,
        SportsAndOutdoorsProduct,
        SportsAndOutdoorsCategory,
        IngestionResult,
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
        .route(
            "/products",
            post(handle_insert_products).put(handle_upsert_products),
        )
        .route("/orders", post(handle_orders))
        .route("/refunds", post(handle_refunds))
        .with_state(state)
}

#[derive(Serialize, Deserialize, ToSchema)]
struct IngestionResult {
    results_affected: u64,
}

impl IngestionResult {
    fn new(results_affected: u64) -> Self {
        Self { results_affected }
    }
}

#[utoipa::path(
    post,
    path = "products",
    request_body = ProductCategory,
    responses(
        (status = 200, body = IngestionResult, description = "Ingest products"),
        (status = "4XX", body = ErrorMessage, description = "Opusi daisy, you messed up"),
        (status = "5XX", body = ErrorMessage, description = "Opusi daisy, we messed up, sorry"),
    )
)]
async fn handle_insert_products(
    state: AppState,
    Extension(current_user): Extension<JWTUserRequest<Tes>>,
    Json(payload): Json<ProductCategory>,
) -> AppResult<IngestionResult> {
    let payload = payload.to_events(current_user.id, current_user.id);
    let result = send_events(Command::Create, &state.events_broker, payload).await?;
    Ok(Json(IngestionResult::new(result)))
}

#[utoipa::path(
    put,
    path = "products",
    request_body = ProductCategory,
    responses(
        (status = 200, body = IngestionResult, description = "Ingest products"),
        (status = "4XX", body = ErrorMessage, description = "Opusi daisy, you messed up"),
        (status = "5XX", body = ErrorMessage, description = "Opusi daisy, we messed up, sorry"),
    )
)]
async fn handle_upsert_products(
    state: AppState,
    Extension(current_user): Extension<JWTUserRequest<Tes>>,
    Json(payload): Json<ProductCategory>,
) -> AppResult<IngestionResult> {
    let payload = payload.to_events(current_user.id, current_user.id);
    let result = send_events(Command::Upsert, &state.events_broker, payload).await?;
    Ok(Json(IngestionResult::new(result)))
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
