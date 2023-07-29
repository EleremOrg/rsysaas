use std::{
    fmt::Write,
    time::{SystemTime, UNIX_EPOCH},
};

use aromatic::Orm;
use rec_rsys::{algorithms::knn::KNN, models::Item, similarity::SimilarityAlgos};
use serde::{Deserialize, Serialize};

use super::requests::{RecommendationRequest, RecommendationTarget};
use crate::data::{
    errors::CRUDError, interface::get_product_comparer, interfaces::db::Manager,
    models::recommendation::RecommendationResponse,
};

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
    pub async fn new(id: u32, title: String, image: String, resume: String) -> Self {
        Recommendation {
            id,
            score: f32::NAN,
            url: String::from(""),
            image,
            title,
            resume,
        }
    }
    pub fn update(&mut self, score: f32, url: String) {
        self.score = score;
        self.url = url;
    }

    pub async fn generate_recommendations(
        request: &RecommendationRequest,
    ) -> Result<Vec<Recommendation>, CRUDError> {
        match request.target {
            RecommendationTarget::Generic => Self::get_generic_recommendations(request).await,
            RecommendationTarget::Product => Self::get_product_recommendations(request).await,
            RecommendationTarget::User => Self::get_user_recommendations(request).await,
        }
    }

    async fn get_generic_recommendations(
        _request: &RecommendationRequest,
    ) -> Result<Vec<Recommendation>, CRUDError> {
        todo!("TODO") //TODO
    }

    async fn get_user_recommendations(
        _request: &RecommendationRequest,
    ) -> Result<Vec<Recommendation>, CRUDError> {
        todo!("TODO") //TODO
    }

    async fn get_product_recommendations(
        request: &RecommendationRequest,
    ) -> Result<Vec<Recommendation>, CRUDError> {
        let mut comparer =
            get_product_comparer(request.get_id().await, request.entity.clone()).await?;

        let sorted_items = Self::calculate_product_recommendations(
            comparer.main.item.clone(),
            &mut comparer.get_items_references().await,
            request.number_recommendations,
        )
        .await;

        let mut result = Vec::new();
        let mut query_values = Vec::new();

        for item in sorted_items {
            let rec_adapter = comparer.references.get(&item.id);
            if let Some(rec_adapter) = rec_adapter {
                let mut selected_recommendation = rec_adapter.recommendation.clone();
                selected_recommendation.update(item.result, Self::get_url("domain", 0));

                query_values.push(format!(
                    "({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {},)",
                    request.request_id,
                    request.request_type,
                    request.customer.id,
                    comparer.main.item.id,
                    comparer.main.entity,
                    selected_recommendation.id,
                    rec_adapter.entity,
                    selected_recommendation.image,
                    selected_recommendation.title,
                    selected_recommendation.resume,
                    item.result,
                    "Cosine",
                    selected_recommendation.url,
                    get_current_time()
                ));
                result.push(selected_recommendation);
            }
        }

        let mut query = Orm::insert(&RecommendationResponse::table().await)
            .set_columns(
                "request_id,
            request_type,
            customer_id,
            main_item_id,
            main_item_entity,
            entity_id,
            entity,
            image,
            title,
            resume,
            score,
            algorithm,
            url,
            created_at",
            )
            .add_many(&query_values.join(","));

        let _ = RecommendationResponse::save_recommendations(&query.ready());

        Ok(result)
    }

    async fn calculate_product_recommendations(
        item: Item,
        references: &Vec<Item>,
        num_recs: u8,
    ) -> Vec<Item> {
        KNN::new(item.clone(), references.clone(), num_recs).result(SimilarityAlgos::Cosine)
    }

    fn get_url(domain: &str, id: u32) -> String {
        format!("my/path/{domain}/{id}/")
    }
}

fn get_current_time() -> String {
    // Get the current system time
    let current_time = SystemTime::now();

    // Get the duration since the Unix epoch (January 1, 1970)
    let duration = current_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    // Extract the number of seconds from the duration
    let seconds = duration.as_secs();

    // Convert the number of seconds to a string
    let mut time_string = String::new();
    write!(&mut time_string, "{}", seconds).expect("Failed to write to string");

    time_string
}
