use crate::data::{Company, Manager, User};
use crate::web::{
    models::RequestModel,
    responses::{not_found, success},
    utils::auth,
    Version,
};
use axum::{
    async_trait,
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use std::collections::HashMap;

pub async fn get_recommendations(Query(payload): Query<RequestModel>) -> Response {
    // auth(payload)
    (StatusCode::OK, Html(format!("{:?}", payload))).into_response()
}

pub async fn get_items(version: Version, Path(id): Path<u32>) -> Response {
    match Company::get(id) {
        Ok(u) => success(u),
        Err(err) => not_found(&id),
    }
}
pub async fn list_items(version: Version, Query(payload): Query<RequestModel>) -> Response {
    (StatusCode::OK, Html(format!("{:?}", payload))).into_response()
}
pub async fn post_items(version: Version) -> Response {
    todo!()
}
pub async fn put_items(version: Version, Path(id): Path<u8>) -> Response {
    todo!()
}
pub async fn patch_items(version: Version, Path(id): Path<u8>) -> Response {
    todo!()
}
pub async fn delete_items(version: Version, Path(id): Path<u8>) -> Response {
    todo!()
}

pub async fn get_users(version: Version, Path(id): Path<u32>) -> Response {
    match User::get(id) {
        Ok(u) => success(u),
        Err(err) => not_found(&id),
    }
}
pub async fn list_users(version: Version, Query(payload): Query<RequestModel>) -> Response {
    (StatusCode::OK, Html(format!("{:?}", payload))).into_response()
}
pub async fn post_users(version: Version) -> Response {
    todo!()
}
pub async fn put_users(version: Version, Path(id): Path<u8>) -> Response {
    todo!()
}
pub async fn patch_users(version: Version, Path(id): Path<u8>) -> Response {
    todo!()
}
pub async fn delete_users(version: Version, Path(id): Path<u8>) -> Response {
    todo!()
}
