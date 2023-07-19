use super::{interface::CustomerInterface, requests::RecommendationRequest};
use crate::data::{errors::CRUDError, interface::get_model_items};
use rec_rsys::{algorithms::knn::KNN, models::Item, similarity::SimilarityAlgos};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recommendation {
    prod_id: u32,
    similarity: f32,
    path: String,
    image: String,
    title: String,
    resume: String,
}

impl Recommendation {
    pub fn new(prod_id: u32, similarity: f32, domain: Arc<String>) -> Self {
        Recommendation {
            prod_id,
            similarity,
            path: Self::generate_path(domain, prod_id),
            image: "https://www.wallstreetmojo.com/wp-content/uploads/2023/04/Current-Cost.png"
                .to_string(),
            title: format!("Title {prod_id}"),
            resume: format!("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s {prod_id}"),
        }
    }

    pub async fn generate_recommendations(
        customer: &CustomerInterface,
        request: &RecommendationRequest,
    ) -> Result<Vec<Recommendation>, CRUDError> {
        let (item, references) = match Self::get_items(&customer, request).await {
            Ok((item, references)) => (item, references),
            Err(e) => return Err(e),
        };
        Ok(Self::calculate_recommendations(
            item,
            references,
            request.number_recommendations,
            customer.domain.clone(),
        )
        .await)
    }

    async fn get_items(
        customer: &CustomerInterface,
        request: &RecommendationRequest,
    ) -> Result<(Item, Vec<Item>), CRUDError> {
        Ok(get_model_items(request.prod_id, request.entity.clone()).await)
    }

    async fn calculate_recommendations(
        item: Item,
        references: Vec<Item>,
        num_recs: u8,
        domain: Arc<String>,
    ) -> Vec<Recommendation> {
        let knn = KNN::new(item, references, num_recs);
        knn.result(SimilarityAlgos::Cosine)
            .into_iter()
            .map(|item| Recommendation::new(item.id, item.result, domain.clone()))
            .collect()
    }

    fn generate_path(domain: Arc<String>, prod_id: u32) -> String {
        format!("my/path/{domain}/{prod_id}/")
    }
}
