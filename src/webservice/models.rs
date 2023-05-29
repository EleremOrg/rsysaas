use crate::recsys::models::Recommendation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RestResponse {
    pub message: String,
    pub recs: Vec<Recommendation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestRequest {
    pub user_id: i16,
    pub prod_id: i16,
    pub num_recs: i8,
    pub token: Option<String>,
}
