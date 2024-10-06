use std::collections::HashMap;

use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::{header::HeaderMap, request::Parts, HeaderValue},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, RequestPartsExt, Router,
};

use hmac::{Hmac, Mac};
use menva::get_env;
use regex::Regex;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::Sha256;

use stefn::{AppError, AppResult, AppState};

struct AppUninstalledPayload {
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
    Json(rec): Json<Value>,
) -> AppResult<Vec<Value>> {
    println!("{rec:?}");
    Ok(Json(vec![]))
}

#[utoipa::path(
    post,
    path = "/products",
    request_body = RecommendationRequest,
    responses(
        (status = 200, body = Vec<Recommendation>, description = "Recommendations for a client"),
        (status = "4XX", body = ErrorMessage, description = "Opusi daisy, you messed up"),
        (status = "5XX", body = ErrorMessage, description = "Opusi daisy, we messed up, sorry"),
    )
)]
pub async fn handle_products(
    headers: HeaderMap,
    state: AppState,
    Json(rec): Json<Value>,
) -> AppResult<Vec<Value>> {
    println!("{rec:?}");
    println!("{headers:?}");
    Ok(Json(vec![]))
}
