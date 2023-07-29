use super::requests::{RecommendationRequest, RecommendationTarget};
use crate::data::{errors::CRUDError, interface::get_product_comparer};
use rec_rsys::{algorithms::knn::KNN, models::Item, similarity::SimilarityAlgos};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recommendation {
    pub id: u32,
    pub score: f32,
    url: String,
    image: String,
    title: String,
    resume: String,
}

impl Recommendation {
    pub async fn default(id: u32, title: String, image: String, resume: String) -> Self {
        Recommendation {
            id,
            score: f32::NAN,
            url: String::from(""),
            image,
            title,
            resume,
        }
    }
    pub fn new(id: u32, score: f32, domain: Arc<String>) -> Self {
        Recommendation {
            id,
            score,
            url: Self::get_url(domain, id),
            image: "https://www.wallstreetmojo.com/wp-content/uploads/2023/04/Current-Cost.png"
                .to_string(),
            title: format!("Title {id}"),
            resume: format!("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s {prod_id}"),
        }
    }

    pub async fn generate_recommendations(
        request: &RecommendationRequest,
    ) -> Result<Vec<Recommendation>, CRUDError> {
        match request.target {
            // RecommendationTarget::Generic => Self::get_generic_recommendations(request).await,
            RecommendationTarget::Product => Self::get_product_recommendations(request).await,
            _ => return Err(CRUDError::JsonError),
            // RecommendationTarget::User => Self::get_user_recommendations(request).await,
        }
    }
    async fn get_product_recommendations(
        request: &RecommendationRequest,
    ) -> Result<Vec<Recommendation>, CRUDError> {
        let comparer = get_product_comparer(request.get_id().await, request.entity.clone()).await?;

        let sorted_items = Self::calculate_product_recommendations(
            &comparer.main.item,
            &comparer.get_items_references().await,
            request.number_recommendations,
        )
        .await;

        Ok()
    }

    async fn calculate_product_recommendations(
        item: &Item,
        references: &Vec<Item>,
        num_recs: u8,
    ) -> Vec<Item> {
        KNN::new(item.clone(), references.clone(), num_recs).result(SimilarityAlgos::Cosine)
    }

    fn get_url(domain: Arc<String>, prod_id: u32) -> String {
        format!("my/path/{domain}/{prod_id}/")
    }
}
