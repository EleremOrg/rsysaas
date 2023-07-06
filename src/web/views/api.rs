use crate::business::{utils::auth, Version};
use crate::data::{Company, DBManager, User};
use crate::web::{
    requests::{PathRequest, QueryRequest, RecommendationQueryRequest},
    responses::{not_found, success, wrong_query},
};
use axum::{
    extract::{Path, Query},
    response::Response,
};

pub async fn get_recommendations(Query(payload): Query<RecommendationQueryRequest>) -> Response {
    auth(payload).await
}

pub async fn get_entities(Path(PathRequest): Path<PathRequest>) -> Response {
    match Company::get(id) {
        Ok(u) => success(u),
        Err(_err) => not_found(&id),
    }
}
pub async fn list_entities(_version: Version, Query(payload): Query<QueryRequest>) -> Response {
    match Company::find(&payload.get_query()) {
        Ok(u) => success(u),
        Err(_err) => wrong_query(&payload.fields),
    }
}
pub async fn post_entities(_version: Version, Query(payload): Query<QueryRequest>) -> Response {
    match Company::create(&payload.get_params()) {
        Ok(u) => success(u),
        Err(_err) => wrong_query(&payload.fields),
    }
}
pub async fn put_entities(
    Path(PathRequest): Path<PathRequest>,
    Query(payload): Query<QueryRequest>,
) -> Response {
    match Company::update(id, &payload.get_params()) {
        Ok(u) => success(u),
        Err(_err) => not_found(&id),
    }
}
pub async fn patch_entities(
    Path(PathRequest): Path<PathRequest>,
    Query(payload): Query<QueryRequest>,
) -> Response {
    match Company::update(id, &payload.get_params()) {
        Ok(u) => success(u),
        Err(_err) => not_found(&id),
    }
}
pub async fn delete_entities(Path(PathRequest): Path<PathRequest>) -> Response {
    match Company::delete(id) {
        Ok(u) => success(u),
        Err(_err) => not_found(&id),
    }
}
