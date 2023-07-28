use std::sync::Arc;

use crate::{
    business::{facade::CustomerFacade, recommendations::Recommendation, versioning::Version},
    web::{
        requests::recommendation::{
            APIRecommendationRequest, EmbedRecommendationRequest, QueryRequest,
        },
        responses::{non_auth, success, wrong_query},
    },
};
use axum::{
    extract::{Query, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    response::Response,
};

pub async fn get_recommendations(
    _version: Version,
    payload: APIRecommendationRequest,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
) -> Response {
    let customer = match CustomerFacade::get_by_token(token.token()).await {
        Ok(customer) => customer,
        Err(_) => return non_auth(),
    };
    match payload.get_request(&customer).await {
        Ok(request) => request.recommend().await,
        Err(err) => err,
    }
}

pub async fn get_embed_recommendations(
    _version: Version,
    payload: EmbedRecommendationRequest,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
) -> Response {
    let customer =
        match CustomerFacade::get_by_public_token_and_domain(token.token(), payload.host.clone())
            .await
        {
            Ok(customer) => customer,
            Err(_) => return non_auth(),
        };
    match payload.get_request(&customer).await {
        Ok(request) => request.recommend().await,
        Err(err) => err,
    }
}
