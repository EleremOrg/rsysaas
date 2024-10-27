use axum::{routing::post, Extension, Json, Router};
use utoipa::{self, OpenApi};

use crate::{
    api_gateway::auth::JWTUser,
    entities::{
        events::Command,
        products::{
            BooksAndMediaCategory, BooksAndMediaProduct, ClothingCategory, ClothingGender,
            ClothingProduct, Order, ProductCategory, Refund, SportsAndOutdoorsCategory,
            SportsAndOutdoorsProduct,
        },
    },
};

use stefn::{AppResult, AppState, ErrorMessage};

use super::{applications::send_events, dtos::IngestionResult};

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
    responses(IngestionResult)),
    security(("token_jwt" = [])),
    tags()
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
    Extension(current_user): JWTUser,
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
    Extension(current_user): JWTUser,
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
