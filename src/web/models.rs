use crate::recsys::models::RecRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestModel {
    pub token: String,
    pub user_id: u32,
    pub prod_id: u32,
    pub num_recs: u8,
}

impl RequestModel {
    pub fn rec_data(self) -> RecRequest {
        RecRequest {
            user_id: self.user_id,
            prod_id: self.prod_id,
            num_recs: self.num_recs,
        }
    }
}
