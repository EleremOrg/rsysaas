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

pub async fn deactivate_user() -> AppResult<Vec<Value>> {
    todo!()
}

pub async fn ingest_webhooks_events() -> AppResult<Vec<Value>> {
    todo!()
}

pub async fn ingest_bulk_operation() -> AppResult<Vec<Value>> {
    todo!()
}
