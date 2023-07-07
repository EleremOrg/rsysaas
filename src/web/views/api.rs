use crate::business::{utils::auth, Version};
use crate::data::Entity;
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
    async {
        match Entity::get(PathRequest.id, &PathRequest.entity).await {
            Ok(u) => success(u),
            Err(_err) => not_found(&PathRequest.id),
        }
    }
    .await
}
pub async fn list_entities(_version: Version, Query(payload): Query<QueryRequest>) -> Response {
    match Entity::find(&payload.get_query()).await {
        Ok(u) => success(u),
        Err(_err) => wrong_query(&payload.fields),
    }
}
pub async fn post_entities(_version: Version, Query(payload): Query<QueryRequest>) -> Response {
    match Entity::create(&payload.get_params()).await {
        Ok(u) => success(u),
        Err(_err) => wrong_query(&payload.fields),
    }
}
pub async fn put_entities(
    Path(PathRequest): Path<PathRequest>,
    Query(payload): Query<QueryRequest>,
) -> Response {
    match Entity::update(id, &payload.get_params()).await {
        Ok(u) => success(u),
        Err(_err) => not_found(&id),
    }
}
pub async fn patch_entities(
    Path(PathRequest): Path<PathRequest>,
    Query(payload): Query<QueryRequest>,
) -> Response {
    match Entity::update(id, &payload.get_params()).await {
        Ok(u) => success(u),
        Err(_err) => not_found(&id),
    }
}
pub async fn delete_entities(Path(PathRequest): Path<PathRequest>) -> Response {
    match Entity::delete(id).await {
        Ok(u) => success(u),
        Err(_err) => not_found(&id),
    }
}
