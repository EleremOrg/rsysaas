use axum::{
    extract::Form,
    http::header::HeaderMap,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{self, OpenApi, ToResponse, ToSchema};

use crate::{
    server::{AppError, AppResult},
    AppState,
};

#[derive(Debug, Deserialize)]
struct Customer {
    id: u64,
    name: String,
}
