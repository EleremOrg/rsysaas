use std::sync::Arc;

use crate::data::facades::db::Manager;
use crate::{
    business::{interface::CustomerInterface, versioning::Version},
    web::{
        requests::query::{ModelQueryRequest, PathRequest},
        responses::{match_error, non_auth},
    },
};

use axum::{
    async_trait,
    extract::{Path, Query, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::HeaderMap,
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
    async fn get(
        Path(path_request): Path<PathRequest>,
        TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    ) -> Response {
        if Self::allow_request(format!("{}s", Self::entity_name()), token).await {
            return match_error(
                <Self as Manager>::get(path_request.id).await,
                &path_request.id,
            )
            .await;
        }
        non_auth()
    }
    async fn list(
        _version: Version,
        Query(payload): Query<ModelQueryRequest>,
        TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    ) -> Response {
        if Self::allow_request(format!("{}s", Self::entity_name()), token).await {
            return match_error(
                <Self as Manager>::find(&Self::default(), payload.get_query()).await,
                &payload.get_query(),
            )
            .await;
        }
        non_auth()
    }
    async fn post(
        _version: Version,
        Query(payload): Query<ModelQueryRequest>,
        TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    ) -> Response {
        if Self::allow_request(format!("{}s", Self::entity_name()), token).await {
            let (fields, values) = payload.get_fields_and_values();
            return match_error(
                <Self as Manager>::create(&fields, &values).await,
                &payload.fields,
            )
            .await;
        }
        non_auth()
    }
    async fn put(
        Path(path_request): Path<PathRequest>,
        Query(payload): Query<ModelQueryRequest>,
        TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    ) -> Response {
        if Self::allow_request(format!("{}s", Self::entity_name()), token).await {
            return match_error(
                <Self as Manager>::update(&Self::default(), path_request.id, &payload.get_params())
                    .await,
                &payload.fields,
            )
            .await;
        }
        non_auth()
    }
    async fn patch(
        Path(path_request): Path<PathRequest>,
        Query(payload): Query<ModelQueryRequest>,
        TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    ) -> Response {
        if Self::allow_request(format!("{}s", Self::entity_name()), token).await {
            return match_error(
                <Self as Manager>::update(&Self::default(), path_request.id, &payload.get_params())
                    .await,
                &payload.fields,
            )
            .await;
        }
        non_auth()
    }
    async fn delete(
        Path(path_request): Path<PathRequest>,
        TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    ) -> Response {
        if Self::allow_request(format!("{}s", Self::entity_name()), token).await {
            return match_error(
                <Self as Manager>::delete(&Self::default(), path_request.id).await,
                &path_request.id,
            )
            .await;
        }
        non_auth()
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

    async fn allow_request(entity: String, token: Authorization<Bearer>) -> bool {
        CustomerInterface::is_allowed(Arc::new(entity), token.token()).await
    }
}
