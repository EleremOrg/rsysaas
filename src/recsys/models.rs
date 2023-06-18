use std::sync::Arc;

use crate::data::items_examples;
use rec_rsys::algorithms::euclidean_knn;
use rec_rsys::models::Item;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecRequest {
    pub user_id: u32,
    pub prod_id: u32,
    pub num_recs: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recommendation {
    prod_id: u32,
    rank: f32,
    path: String,
}

impl Recommendation {
    pub fn new(prod_id: u32, rank: f32, domain: Arc<str>) -> Self {
        Recommendation {
            prod_id,
            rank,
            path: Recommendation::generate_path(domain, prod_id),
        }
    }
    pub fn generate_recommendations(
        domain: Arc<str>,
        item: Item,
        num_recs: u8,
    ) -> Vec<Recommendation> {
        euclidean_knn(item, items_examples(), num_recs)
            .into_iter()
            .map(|item| Recommendation::new(item.id, item.result, domain.clone()))
            .collect()
    }
    fn generate_path(domain: Arc<str>, prod_id: u32) -> String {
        format!("my/path/{domain}/{prod_id}/")
    }
}
