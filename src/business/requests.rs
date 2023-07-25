use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{
    data::{
        facades::db::Manager,
        models::requests::{APIRecommendationRequest, EmbedRecommendationRequest},
    },
    web::requests::{APIRecommendationQueryRequest, EmbedRecommendationQueryRequest, QueryRequest},
};

use super::interface::CustomerInterface;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecommendationRequest {
    pub prod_id: Option<u32>,
    pub user_id: Option<u32>,
    pub number_recommendations: u8,
    pub entity: Arc<String>,
    pub customer: CustomerInterface,
    pub is_for: RecommendFor,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum RecommendFor {
    User,
    Product,
    Generic,
}

impl RecommendFor {}

impl RecommendationRequest {
    pub async fn get_id(&self) -> u32 {
        match self.is_for {
            RecommendFor::Generic => 0,
            RecommendFor::User => self.user_id.unwrap(),
            RecommendFor::Product => self.prod_id.unwrap(),
        }
    }

    async fn save_embed_query(payload: &EmbedRecommendationQueryRequest) {
        let (fields, values) = payload.get_fields_and_values().await;
        //TODO: use result, maybe move it into a form
        EmbedRecommendationRequest::create(&fields, &values);
    }
    async fn save_api_query(payload: &APIRecommendationQueryRequest) {
        let (fields, values) = payload.get_fields_and_values().await;
        //TODO: use result, maybe move it into a form
        APIRecommendationRequest::create(&fields, &values);
    }
    pub async fn from_embed(
        customer: &CustomerInterface,
        payload: &EmbedRecommendationQueryRequest,
    ) -> Self {
        RecommendationRequest {
            prod_id: payload.prod_id,
            user_id: payload.user_id,
            number_recommendations: payload.number_recommendations.unwrap_or(5),
            entity: payload.entity.clone(),
            customer: customer.clone(),
            is_for: RecommendFor::Generic, //TODO: change
        }
    }
    pub async fn from_api(
        customer: &CustomerInterface,
        payload: &APIRecommendationQueryRequest,
    ) -> Self {
        RecommendationRequest {
            prod_id: payload.prod_id,
            user_id: payload.user_id,
            number_recommendations: payload.number_recommendations.unwrap_or(5),
            entity: payload.entity.clone(),
            customer: customer.clone(),
            is_for: RecommendFor::Generic, //TODO: change
        }
    }
}
