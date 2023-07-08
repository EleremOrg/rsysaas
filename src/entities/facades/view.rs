use super::db::Manager;
use crate::{
    business::{auth::auth, versioning::Version},
    web::{
        requests::{PathRequest, QueryRequest},
        responses::match_error,
    },
};
use axum::{
    async_trait,
    extract::{Path, Query},
    response::Response,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait View<'a>
where
    Self: Manager<'a> + Default + Sync + Send + Unpin + Deserialize<'a> + Serialize + 'static,
{
    fn entity_name() -> String {
        std::any::type_name::<Self>()
            .rsplit("::")
            .next()
            .unwrap()
            .to_lowercase()
    }
    fn base_path() -> String {
        format!("/{}s/", Self::entity_name())
    }
    async fn get(Path(path_request): Path<PathRequest>) -> Response {
        match_error(
            <Self as Manager>::get(&Self::default(), path_request.id).await,
            &path_request.id,
        )
        .await
    }
    async fn list(_version: Version, Query(payload): Query<QueryRequest>) -> Response {
        match_error(
            <Self as Manager>::find(&Self::default(), &payload.get_query()).await,
            &payload.get_query(),
        )
        .await
    }
    async fn post(_version: Version, Query(payload): Query<QueryRequest>) -> Response {
        match_error(
            <Self as Manager>::create(&Self::default(), &payload.get_params()).await,
            &payload.fields,
        )
        .await
    }
    async fn put(
        Path(path_request): Path<PathRequest>,
        Query(payload): Query<QueryRequest>,
    ) -> Response {
        match_error(
            <Self as Manager>::update(&Self::default(), path_request.id, &payload.get_params())
                .await,
            &payload.fields,
        )
        .await
    }
    async fn patch(
        Path(path_request): Path<PathRequest>,
        Query(payload): Query<QueryRequest>,
    ) -> Response {
        match_error(
            <Self as Manager>::update(&Self::default(), path_request.id, &payload.get_params())
                .await,
            &payload.fields,
        )
        .await
    }
    async fn delete(Path(path_request): Path<PathRequest>) -> Response {
        match_error(
            <Self as Manager>::delete(&Self::default(), path_request.id).await,
            &path_request.id,
        )
        .await
    }
    fn routes() -> Router {
        Router::new()
            .route(&Self::base_path(), get(<Self as View>::list))
            .route(
                &format!("{}:id/", Self::base_path()),
                get(<Self as View>::get)
                    .post(<Self as View>::post)
                    .put(<Self as View>::put)
                    .patch(<Self as View>::patch)
                    .delete(<Self as View>::delete),
            )
    }
}
