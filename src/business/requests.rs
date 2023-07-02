use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct RecommendationRequest {
    pub user_id: u32,
    pub prod_id: u32,
    pub num_recs: u8,
}

impl RecommendationRequest {
    pub fn new(user_id: u32, prod_id: u32, num_recs: u8) -> Self {
        RecommendationRequest {
            user_id: user_id,
            prod_id: prod_id,
            num_recs: num_recs,
        }
    }
}
