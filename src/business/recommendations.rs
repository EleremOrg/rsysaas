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
        request: &RecommendationRequest,
    ) -> Result<Vec<Recommendation>, CRUDError> {
        match request.target {
            RecommendationTarget::User => Self::get_user_recommendations(request).await,
            RecommendationTarget::Product => Self::get_product_recommendations(request).await,
            RecommendationTarget::Generic => Self::get_generic_recommendations(request).await,
        }
    }

    async fn get_user_recommendations(
        request: &RecommendationRequest,
    ) -> Result<Vec<Recommendation>, CRUDError> {
        let (item, references) = match get_user_items(request.get_id().await).await {
            Ok((item, references)) => (item, references),
            Err(e) => return Err(e),
        };
        Ok(Self::calculate_product_recommendations(
            item,
            references,
            request.number_recommendations,
            request.customer.domain.clone(),
        )
        .await)
    }

    async fn get_generic_recommendations(
        request: &RecommendationRequest,
    ) -> Result<Vec<Recommendation>, CRUDError> {
        let items = match get_generic_items().await {
            Ok(items) => items,
            Err(err) => return Err(err),
        };
        Ok(items
            .iter()
            .map(|item| Recommendation::new(item.id, item.result, request.customer.domain.clone()))
            .collect::<Vec<Recommendation>>())
    }

    async fn get_product_recommendations(
        request: &RecommendationRequest,
    ) -> Result<Vec<Recommendation>, CRUDError> {
        let (item, references) =
            match get_product_items(request.get_id().await, request.entity.clone()).await {
                Ok((item, references)) => (item, references),
                Err(e) => return Err(e),
            };
        Ok(Self::calculate_product_recommendations(
            item,
            references,
            request.number_recommendations,
            request.customer.domain.clone(),
        )
        .await)
    }

    async fn calculate_product_recommendations(
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
