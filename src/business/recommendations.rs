use super::requests::{RecommendationRequest, RecommendationTarget};
use crate::data::{
    errors::CRUDError,
    interface::{get_generic_items, get_product_items, get_user_items},
};
use rec_rsys::{algorithms::knn::KNN, models::Item, similarity::SimilarityAlgos};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recommendation {
    prod_id: u32,
    score: f32,
    url: String,
    image: String,
    title: String,
    resume: String,
}

impl Recommendation {
    pub fn new(prod_id: u32, score: f32, domain: Arc<String>) -> Self {
        Recommendation {
            prod_id,
            score,
            url: Self::get_url(domain, prod_id),
            image: "https://www.wallstreetmojo.com/wp-content/uploads/2023/04/Current-Cost.png"
                .to_string(),
            title: format!("Title {prod_id}"),
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
        let (item, references) =
            match get_product_items(request.get_id().await, request.entity.clone()).await {
                Ok((item, references)) => (item, references),
                Err(e) => return Err(e),
            };
        let final_items = Self::calculate_product_recommendations(
            item,
            references,
            request.number_recommendations,
        )
        .await;
        Ok(vec![Recommendation::new(
            2,
            2.,
            Arc::new("final_items".to_string()),
        )])
    }

    async fn calculate_product_recommendations(
        item: Item,
        references: Vec<Item>,
        num_recs: u8,
    ) -> Vec<Item> {
        let knn = KNN::new(item, references, num_recs);
        knn.result(SimilarityAlgos::Cosine)
    }

    fn get_url(domain: Arc<String>, prod_id: u32) -> String {
        format!("my/path/{domain}/{prod_id}/")
    }
}
