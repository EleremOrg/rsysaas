use crate::business::{utils::auth, Version};
use crate::data::{Company, Manager, User};
use crate::web::{
    requests::RequestModel,
    responses::{not_found, success},
};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use std::env;

pub async fn get_recommendations(Query(payload): Query<RequestModel>) -> Response {
    auth(payload).await
}

pub async fn get_items(version: Version, Path((_, id)): Path<(String, u32)>) -> Response {
    let var_value = env::var("VAR_NAME").unwrap();
    println!("{:?}", var_value);
    match Company::get(id) {
        Ok(u) => success(u),
        Err(err) => not_found(&id),
    }
}
pub async fn list_items(version: Version) -> Response {
    (StatusCode::OK, Html(format!("{:?}", version))).into_response()
}
pub async fn post_items(version: Version) -> Response {
    todo!()
}
pub async fn put_items(version: Version, Path((_, id)): Path<(String, u32)>) -> Response {
    todo!()
}
pub async fn patch_items(version: Version, Path((_, id)): Path<(String, u32)>) -> Response {
    todo!()
}
pub async fn delete_items(version: Version, Path((_, id)): Path<(String, u32)>) -> Response {
    todo!()
}

pub async fn get_users(version: Version, Path((_, id)): Path<(String, u32)>) -> Response {
    let var_value = env::var("VAR_NAME").unwrap();
    println!("{:?}", var_value);
    match User::get(id) {
        Ok(u) => success(u),
        Err(err) => not_found(&id),
    }
}
pub async fn list_users(version: Version) -> Response {
    (StatusCode::OK, Html(format!("{:?}", version))).into_response()
}
pub async fn post_users(version: Version) -> Response {
    todo!()
}
pub async fn put_users(version: Version, Path((_, id)): Path<(String, u32)>) -> Response {
    todo!()
}
pub async fn patch_users(version: Version, Path((_, id)): Path<(String, u32)>) -> Response {
    todo!()
}
pub async fn delete_users(version: Version, Path((_, id)): Path<(String, u32)>) -> Response {
    todo!()
}
