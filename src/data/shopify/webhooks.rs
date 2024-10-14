use axum::{http::header::HeaderMap, Json};
use serde::Deserialize;
use serde_json::Value;

use stefn::{AppResult, AppState};

/// https://shopify.dev/docs/api/admin-graphql/2024-10/objects/BulkOperation
#[derive(Debug, Deserialize)]
pub struct BulkOperation {
    id: String,
    completed_at: String,
    created_at: String,
    error_code: Option<String>,
    object_count: u64,
    status: String,
    type_: String,
    url: String,
}

pub async fn handle_bulk_operations(
    headers: HeaderMap,
    state: AppState,
    Json(rec): Json<BulkOperation>,
) -> AppResult<Vec<Value>> {
    // TODO: when bulk operation ends read file
    println!("{rec:?}");
    Ok(Json(vec![]))
}

#[derive(Debug, Deserialize)]
pub struct AppUninstalledPayload {
    id: u64,
    name: String,
    email: String,
    domain: Option<String>,
    province: String,
    country: String,
    address1: String,
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
