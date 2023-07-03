use crate::business::{utils::auth, Version};
use crate::data::{Company, Manager, User};
use crate::web::{
    requests::{QueryRequest, RecommendationQueryRequest},
    responses::{not_found, success, wrong_query},
};
use axum::{
    extract::{Path, Query},
    response::Response,
};

pub async fn get_recommendations(Query(payload): Query<RecommendationQueryRequest>) -> Response {
    auth(payload).await
}

pub async fn get_items(_version: Version, Path((_, id)): Path<(String, u32)>) -> Response {
    match Company::get(id) {
        Ok(u) => success(u),
        Err(err) => not_found(&id),
    }
}
pub async fn list_items(_version: Version, Query(payload): Query<QueryRequest>) -> Response {
    match Company::find(&payload.get_query()) {
        Ok(u) => success(u),
        Err(err) => wrong_query(&payload.fields),
    }
}
pub async fn post_items(_version: Version, Query(payload): Query<QueryRequest>) -> Response {
    match Company::create(&payload.get_params()) {
        Ok(u) => success(u),
        Err(err) => wrong_query(&payload.fields),
    }
}
pub async fn put_items(
    _version: Version,
    Path((_, id)): Path<(String, u32)>,
    Query(payload): Query<QueryRequest>,
) -> Response {
    match Company::update(id, &payload.get_params()) {
        Ok(u) => success(u),
        Err(err) => not_found(&id),
    }
}
pub async fn patch_items(
    _version: Version,
    Path((_, id)): Path<(String, u32)>,
    Query(payload): Query<QueryRequest>,
) -> Response {
    match Company::update(id, &payload.get_params()) {
        Ok(u) => success(u),
        Err(err) => not_found(&id),
    }
}
pub async fn delete_items(_version: Version, Path((_, id)): Path<(String, u32)>) -> Response {
    match Company::delete(id) {
        Ok(u) => success(u),
        Err(err) => not_found(&id),
    }
}

pub async fn get_users(_version: Version, Path((_, id)): Path<(String, u32)>) -> Response {
    match User::get(id) {
        Ok(u) => success(u),
        Err(err) => not_found(&id),
    }
}
pub async fn list_users(_version: Version, Query(payload): Query<QueryRequest>) -> Response {
    match User::find(&payload.get_query()) {
        Ok(u) => success(u),
        Err(err) => wrong_query(&payload.fields),
    }
}
pub async fn post_users(_version: Version, Query(payload): Query<QueryRequest>) -> Response {
    match User::create(&payload.get_params()) {
        Ok(u) => success(u),
        Err(err) => wrong_query(&payload.fields),
    }
}
pub async fn put_users(
    _version: Version,
    Path((_, id)): Path<(String, u32)>,
    Query(payload): Query<QueryRequest>,
) -> Response {
    match User::update(id, &payload.get_params()) {
        Ok(u) => success(u),
        Err(err) => wrong_query(&payload.fields),
    }
}
pub async fn patch_users(
    _version: Version,
    Path((_, id)): Path<(String, u32)>,
    Query(payload): Query<QueryRequest>,
) -> Response {
    match User::update(id, &payload.get_params()) {
        Ok(u) => success(u),
        Err(err) => wrong_query(&payload.fields),
    }
}
pub async fn delete_users(_version: Version, Path((_, id)): Path<(String, u32)>) -> Response {
    match User::delete(id) {
        Ok(u) => success(u),
        Err(err) => not_found(&id),
    }
}
