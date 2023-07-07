use super::db::Manager;
use axum::async_trait;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[async_trait]
trait View<'a>
where
    Self: Manager<'a> + Default + Sync + Send + Unpin + Deserialize<'a> + Serialize,
{
    fn path() -> String {
        std::any::type_name::<Self>()
            .rsplit("::")
            .next()
            .unwrap()
            .to_lowercase()
    }
    async fn get() -> Response {
        let entity = Self::default();
        let data = <Self as Manager>::get(&entity, 32 as u32).await;
        match data {
            Ok(data) => Json(data).into_response(),
            Err(_) => (StatusCode::OK, Json(json!({ "data": "Ok(data)" }))).into_response(),
        }
    }
    // pub async fn list() -> Response {
    //     match Entity::find(&payload.get_query()).await {
    //         Ok(u) => success(u),
    //         Err(_err) => wrong_query(&payload.fields),
    //     }
    // }
    // pub async fn post() -> Response {
    //     match Entity::create(&payload.get_params()).await {
    //         Ok(u) => success(u),
    //         Err(_err) => wrong_query(&payload.fields),
    //     }
    // }
    // pub async fn put() -> Response {
    //     match Entity::update(id, &payload.get_params()).await {
    //         Ok(u) => success(u),
    //         Err(_err) => not_found(&id),
    //     }
    // }
    // pub async fn patch() -> Response {
    //     match Entity::update(id, &payload.get_params()).await {
    //         Ok(u) => success(u),
    //         Err(_err) => not_found(&id),
    //     }
    // }
    // pub async fn delete() -> Response {
    //     match Entity::delete(id).await {
    //         Ok(u) => success(u),
    //         Err(_err) => not_found(&id),
    //     }
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     struct User;

//     impl View<'_> for User {}

//     #[test]
//     fn test_path() {
//         assert_eq!(&User::path(), "user");
//     }
// }
