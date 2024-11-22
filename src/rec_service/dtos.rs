use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToResponse, ToSchema)]
pub struct Recommendation {
    id: String,
    score: f32,
}

impl Recommendation {
    pub fn new(id: String, score: f32) -> Self {
        Self { id, score }
    }
}
