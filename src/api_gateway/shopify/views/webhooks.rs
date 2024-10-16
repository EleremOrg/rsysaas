use axum::{http::header::HeaderMap, Json};

use serde_json::Value;

use stefn::{AppResult, AppState};

use crate::api_gateway::shopify::models::webhooks::{AppUninstalledPayload, BulkOperation};

pub async fn handle_bulk_operations(
    headers: HeaderMap,
    state: AppState,
    Json(rec): Json<BulkOperation>,
) -> AppResult<Vec<Value>> {
    // TODO: when bulk operation ends read file
    println!("{rec:?}");
    Ok(Json(vec![]))
}

pub async fn handle_app(
    headers: HeaderMap,
    state: AppState,
    Json(rec): Json<AppUninstalledPayload>,
) -> AppResult<Vec<Value>> {
    println!("{rec:?}");
    Ok(Json(vec![]))
}

pub async fn handle_products(
    headers: HeaderMap,
    state: AppState,
    Json(rec): Json<Value>,
) -> AppResult<Vec<Value>> {
    //TODO: add type to object recieved.
    println!("{rec:?}");
    println!("{headers:?}");
    Ok(Json(vec![]))
}
