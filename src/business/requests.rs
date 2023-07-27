use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    data::{
        facades::db::Manager,
        models::requests::{APIRecommendationRequestModel, EmbedRecommendationRequestModel},
    },
    web::requests::recommendation::{
        APIRecommendationRequest, EmbedRecommendationRequest, QueryRequest,
    },
};

use super::interface::CustomerInterface;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecommendationRequest {
    pub prod_id: Option<u32>,
    pub user_id: Option<u32>,
    pub number_recommendations: u8,
    pub entity: Arc<String>,
    pub customer: CustomerInterface,
    pub target: RecommendationTarget,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum RecommendationTarget {
    User,
    Product,
    Generic,
}

impl RecommendationTarget {
    pub async fn get(target: &str) -> Result<Self, ()> {
        match target {
            "user" => Ok(RecommendationTarget::User),
            "product" => Ok(RecommendationTarget::Product),
            "generic" => Ok(RecommendationTarget::Generic),
            _ => Err(()),
        }
    }
}

impl RecommendationRequest {
    pub async fn get_id(&self) -> u32 {
        match self.target {
            RecommendationTarget::Generic => 0,
            RecommendationTarget::User => self.user_id.unwrap(),
            RecommendationTarget::Product => self.prod_id.unwrap(),
        }
    }

    pub async fn save_embed_query(payload: &EmbedRecommendationRequest) {
        let (fields, values) = payload.get_fields_and_values().await;
        let _ = EmbedRecommendationRequestModel::create(&fields, &values).await;
    }

    pub async fn save_api_query(payload: &APIRecommendationRequest) {
        let (fields, values) = payload.get_fields_and_values().await;
        let _ = APIRecommendationRequestModel::create(&fields, &values).await;
    }
}
