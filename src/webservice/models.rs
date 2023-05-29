use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RestResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestRequest {
    foo: String,
}
