use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

#[derive(Serialize, Deserialize, ToResponse, ToSchema)]
pub struct IngestionResult {
    results_affected: u64,
}

impl IngestionResult {
    pub fn new(results_affected: u64) -> Self {
        Self { results_affected }
    }
}
