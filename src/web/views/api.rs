use super::models::RequestModel;
use crate::web::{models::RequestModel, utils::auth};
use axum::response::{Html, IntoResponse, Response};
use axum::{
    async_trait,
    extract::{FromRequestParts, Path, Query},
    http::{request::Parts, StatusCode},
    response::{Html, IntoResponse, Response},
    RequestPartsExt,
};
use axum::{extract::Query, http::StatusCode};
use serde::Serialize;
use std::collections::HashMap;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn handle_recommendations(Query(payload): Query<RequestModel>) -> Response {
    auth(payload)
}

pub async fn handle_items(Query(payload): Query<RequestModel>) -> Response {
    auth(payload)
}

pub async fn handle_users(Query(payload): Query<RequestModel>) -> Response {
    auth(payload)
}

#[async_trait]
trait View {
    type ListQuery: Serialize + Send + Sync;

    async fn get() -> Response {
        todo!()
    }
    async fn list(Query(payload): Query<Self::ListQuery>) -> Response {
        todo!()
    }
    async fn post() -> Response {
        todo!()
    }
    async fn put() -> Response {
        todo!()
    }
    async fn patch() -> Response {
        todo!()
    }
    async fn delete() -> Response {
        todo!()
    }
}
