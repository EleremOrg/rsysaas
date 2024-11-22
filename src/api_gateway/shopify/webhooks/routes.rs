use axum::Json;

use serde_json::Value;

use stefn::AppResult;

use super::dtos::{AppUninstalledPayload, BulkOperation};

pub async fn handle_bulk_operations(Json(rec): Json<BulkOperation>) -> AppResult<Vec<Value>> {
    // TODO: when bulk operation ends read file
    println!("{rec:?}");
    Ok(Json(vec![]))
}

pub async fn handle_app(Json(rec): Json<AppUninstalledPayload>) -> AppResult<Vec<Value>> {
    println!("{rec:?}");
    Ok(Json(vec![]))
}

pub async fn handle_products(Json(rec): Json<Value>) -> AppResult<Vec<Value>> {
    //TODO: add type to object recieved.
    println!("{rec:?}");
    Ok(Json(vec![]))
}
