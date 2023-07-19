use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{
    data::{
        facades::db::Manager,
        models::requests::{APIRecommendationRequest, EmbedRecommendationRequest},
    },
    web::requests::{APIRecommendationQueryRequest, EmbedRecommendationQueryRequest},
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
        EmbedRecommendationRequest::create(&Self::struct_to_hashmap(payload).await);
    }
    async fn save_api_query(payload: &APIRecommendationQueryRequest) {
        APIRecommendationRequest::create(&Self::struct_to_hashmap(payload).await);
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
    async fn struct_to_hashmap<T>(data: &T) -> HashMap<String, String>
    where
        T: Serialize,
    {
        let mut result = HashMap::new();
        let parameters = match serde_json::to_value(data) {
            Ok(obj) => obj,
            _ => panic!("Unexpected JSON value"),
        };
        let obj = match parameters.as_object() {
            Some(val) => val,
            None => panic!("Unexpected JSON value"),
        };
        for (key, value) in obj {
            result.insert(key.to_owned(), format!("{value}"));
        }
        result
    }
}
