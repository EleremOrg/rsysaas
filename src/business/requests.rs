use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::web::requests::{EmbedRecommendationQueryRequest, RecommendationQueryRequest};

use super::interface::CustomerInterface;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecommendationRequest {
    pub prod_id: Option<u32>,
    pub user_id: Option<u32>,
    pub number_recommendations: u8,
    pub entity: Arc<String>,
    pub customer: CustomerInterface,
}

impl RecommendationRequest {
    async fn save_embed_query() {}
    async fn save_api_query() {}
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
        }
    }
    pub async fn from_api(
        customer: &CustomerInterface,
        payload: &RecommendationQueryRequest,
    ) -> Self {
        RecommendationRequest {
            prod_id: payload.prod_id,
            user_id: payload.user_id,
            number_recommendations: payload.num_recs.unwrap_or(5),
            entity: payload.entity.clone(),
            customer: customer.clone(),
        }
    }
}
