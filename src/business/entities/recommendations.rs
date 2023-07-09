use crate::business::requests::RecommendationRequest;
use crate::data::errors::CRUDError;
use rec_rsys::{algorithms::knn::KNN, models::Item, similarity::SimilarityAlgos};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
            path: Self::generate_path(domain, prod_id),
        }
    }
    pub fn generate_recommendations(
        domain: Arc<str>,
        item: Item,
        references: Vec<Item>,
        num_recs: u8,
    ) -> Vec<Recommendation> {
        let knn = KNN::new(item, references, num_recs);
        let mut result: Vec<Recommendation> = Vec::new();
        [
            Self::calculate_recommendations(SimilarityAlgos::Cosine, &knn, domain.clone()),
            Self::calculate_recommendations(SimilarityAlgos::AdjustedCosine, &knn, domain.clone()),
            Self::calculate_recommendations(SimilarityAlgos::Euclidean, &knn, domain.clone()),
            Self::calculate_recommendations(SimilarityAlgos::Spearman, &knn, domain.clone()),
            Self::calculate_recommendations(
                SimilarityAlgos::PearsonCorrelation,
                &knn,
                domain.clone(),
            ),
        ]
        .iter()
        .for_each(|f| result.extend(f.iter().cloned()));
        result
    }
    fn calculate_recommendations(
        algorithm: SimilarityAlgos,
        knn: &KNN,
        domain: Arc<str>,
    ) -> Vec<Recommendation> {
        knn.result(algorithm)
            .into_iter()
            .map(|item| Recommendation::new(item.id, item.result, domain.clone()))
            .collect()
    }
    fn generate_path(domain: Arc<str>, prod_id: u32) -> String {
        format!("my/path/{domain}/{prod_id}/")
    }
}
