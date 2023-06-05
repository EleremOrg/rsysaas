use crate::recsys::models::{RecRequest, Recommendation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseModel {
    pub recs: Vec<Recommendation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestModel {
    pub token: String,
    pub rec_data: RecRequest,
}
