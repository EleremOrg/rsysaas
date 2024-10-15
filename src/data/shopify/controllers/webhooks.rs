use axum::{http::header::HeaderMap, Json};
use serde::Deserialize;
use serde_json::Value;

use stefn::{AppResult, AppState};

pub async fn deactivate_user() -> AppResult<Vec<Value>> {
    todo!()
}

pub async fn ingest_webhooks_events() -> AppResult<Vec<Value>> {
    todo!()
}

pub async fn ingest_bulk_operation() -> AppResult<Vec<Value>> {
    todo!()
}
