use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestModel {
    pub token: String,
    pub user_id: u32,
    pub prod_id: u32,
    pub num_recs: u8,
}
