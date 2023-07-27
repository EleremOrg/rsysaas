use std::sync::Arc;

use crate::{
    business::{
        interface::CustomerInterface, recommendations::Recommendation, versioning::Version,
    },
    data::errors::CRUDError,
    web::{
        requests::recommendation::{
            APIRecommendationRequest, EmbedRecommendationRequest, QueryRequest,
        },
        responses::{non_auth, our_fault, success, wrong_query},
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
    let customer = match CustomerInterface::get_by_token(token.token()).await {
        Ok(customer) => customer,
        Err(_) => return non_auth(),
    };
    if customer.models_related.contains(payload.entity.as_ref()) {
        let request = match payload.final_request(&customer).await {
            Ok(request) => request,
            Err(_) => return wrong_query("target missing"),
        };
        return match customer.get_recommendations(&request).await {
            Ok(recs) => success(recs),
            Err(err) => match err {
                CRUDError::NotFound => wrong_query(&payload),
                CRUDError::MaxRetry => our_fault(),
                _ => our_fault(),
            },
        };
    }
    wrong_query(&format!("wrong entity {:?}", *payload.entity))
}

pub async fn get_embed_recommendations(
    Query(payload): Query<EmbedRecommendationRequest>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
) -> Response {
    let customer = match CustomerInterface::get_by_public_token_and_domain(
        token.token(),
        payload.host.clone(),
    )
    .await
    {
        Ok(customer) => customer,
        Err(_) => return non_auth(),
    };
    if customer.models_related.contains(payload.entity.as_ref()) {
        let request = match payload.final_request(&customer).await {
            Ok(request) => request,
            Err(_) => return wrong_query("target missing"),
        };
        // return match customer
        //     .get_recommendations(&RecommendationRequest::from_embed(&customer, &payload).await)
        //     .await
        // {
        //     Ok(recs) => success(recs),
        //     Err(err) => match err {
        //         CRUDError::NotFound => wrong_query(&payload),
        //         CRUDError::MaxRetry => our_fault(),
        //         _ => our_fault(),
        //     },
        // };
        return success([
            Recommendation::new(1, 0.98, Arc::new(String::from("invfin"))),
            Recommendation::new(2, 0.88, Arc::new(String::from("invfin"))),
            Recommendation::new(3, 0.78, Arc::new(String::from("invfin"))),
            Recommendation::new(4, 0.68, Arc::new(String::from("invfin"))),
        ]);
    };
    wrong_query(&payload.entity)
}
