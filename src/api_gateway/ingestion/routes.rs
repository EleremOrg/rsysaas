use axum::{extract::State, routing::post, Extension, Json, Router};
use utoipa::{self, OpenApi};

use crate::{
    api_gateway::auth::JWTUser,
    entities::{
        events::Command,
        products::{
            ClothingCategory, ClothingGender, ClothingProduct, Order, ProductPayload, Refund,
            SportsAndOutdoorsCategory, SportsAndOutdoorsProduct,
        },
    },
};

use stefn::{APIState, AppResult, Broker, ErrorMessage};

use super::{applications::send_events, dtos::IngestionResult};

#[derive(OpenApi)]
#[openapi(
    paths(handle_insert_products, handle_upsert_products, handle_orders, handle_refunds),
    components(schemas(
        ProductPayload,
        ClothingProduct,
        ClothingCategory,
        ClothingGender,
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

pub fn routes(state: APIState) -> Router<APIState> {
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
    path = "/products",
    request_body = ProductPayload,
    responses(
        (status = 200, body = IngestionResult, description = "Ingest products"),
        (status = "4XX", body = ErrorMessage, description = "Oupsi daisy, you messed up"),
        (status = "5XX", body = ErrorMessage, description = "Oupsi daisy, we messed up, sorry"),
    )
)]
async fn handle_insert_products(
    events_broker: State<Broker>,
    Extension(current_user): JWTUser,
    Json(payload): Json<ProductPayload>,
) -> AppResult<IngestionResult> {
    let payload = payload.to_events(current_user.id, current_user.id);
    let result = send_events(Command::Create, &events_broker, payload).await?;
    Ok(Json(IngestionResult::new(result)))
}

#[utoipa::path(
    put,
    path = "/products",
    request_body = ProductPayload,
    responses(
        (status = 200, body = IngestionResult, description = "Ingest products"),
        (status = "4XX", body = ErrorMessage, description = "Oupsi daisy, you messed up"),
        (status = "5XX", body = ErrorMessage, description = "Oupsi daisy, we messed up, sorry"),
    )
)]
async fn handle_upsert_products(
    events_broker: State<Broker>,
    Extension(current_user): JWTUser,
    Json(payload): Json<ProductPayload>,
) -> AppResult<IngestionResult> {
    let payload = payload.to_events(current_user.id, current_user.id);
    let result = send_events(Command::Upsert, &events_broker, payload).await?;
    Ok(Json(IngestionResult::new(result)))
}

#[utoipa::path(
    post,
    path = "/orders",
    request_body = Order,
    responses(
        (status = 200, body = IngestionResult, description = "Ingest orders"),
        (status = "4XX", body = ErrorMessage, description = "Oupsi daisy, you messed up"),
        (status = "5XX", body = ErrorMessage, description = "Oupsi daisy, we messed up, sorry"),
    )
)]
async fn handle_orders(_state: State<APIState>, Json(_order): Json<Order>) -> AppResult<usize> {
    Ok(Json(200))
}

#[utoipa::path(
    post,
    path = "/refunds",
    request_body = Refund,
    responses(
        (status = 200, body = IngestionResult, description = "Ingest refunds"),
        (status = "4XX", body = ErrorMessage, description = "Oupsi daisy, you messed up"),
        (status = "5XX", body = ErrorMessage, description = "Oupsi daisy, we messed up, sorry"),
    )
)]
async fn handle_refunds(_state: State<APIState>, Json(_rec): Json<Refund>) -> AppResult<usize> {
    //TODO
    Ok(Json(200))
}
