use crate::web::RecommendationQueryRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecommendationRequest {
    pub user_id: u32,
    pub prod_id: u32,
    pub entity: String,
    pub num_recs: u8,
}

impl RecommendationRequest {
    pub fn new(recommendation_query: RecommendationQueryRequest) -> Self {
        RecommendationRequest {
            user_id: recommendation_query.user_id,
            prod_id: recommendation_query.prod_id,
            num_recs: recommendation_query.num_recs,
        }
    }
}