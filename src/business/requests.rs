use std::sync::Arc;

use crate::web::requests::RecommendationQueryRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecommendationRequest {
    pub user_id: u32,
    pub prod_id: u32,
    pub entity: Arc<String>,
    pub num_recs: u8,
}

impl RecommendationRequest {
    pub fn new(recommendation_query: RecommendationQueryRequest) -> Self {
        RecommendationRequest {
            entity: recommendation_query.entity,
            user_id: recommendation_query.user_id,
            prod_id: recommendation_query.prod_id,
            num_recs: recommendation_query.num_recs,
        }
    }
}
