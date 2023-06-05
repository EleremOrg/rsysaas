use crate::data::{items_examples, Company};
use rec_rsys::algorithms::euclidean_knn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecRequest {
    pub user_id: u32,
    pub prod_id: u32,
    pub num_recs: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recommendation {
    prod_id: u32,
    confidence: f64,
    path: String,
}

impl Recommendation {
    pub fn new(prod_id: u32, confidence: f64, domain: String) -> Self {
        Recommendation {
            prod_id,
            confidence,
            path: Recommendation::generate_path(domain, prod_id),
        }
    }
    pub fn get_recommendations(domain: String, item: Item, num_recs: usize) -> Vec<Recommendation> {
        euclidean_knn(items_examples(), items_examples(), num_recs)
            .into_iter()
            .map(|(conf, id)| Recommendation::new(id as u32, conf, domain))
            .collect()
    }
    fn generate_recommendations() {}
    fn generate_path(domain: String, prod_id: u32) -> String {
        format!("my/path/{domain}/{prod_id}/")
    }
}
