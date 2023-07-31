use std::{
    fmt::Write,
    time::{SystemTime, UNIX_EPOCH},
};

use aromatic::Orm;
use rec_rsys::{algorithms::knn::KNN, models::Item, similarity::SimilarityAlgos};
use serde::{Deserialize, Serialize};

use super::{
    interface::{RecommendationAdapter, RecommendationComparer},
    requests::{RecommendationRequest, RecommendationTarget},
    ulid::Ulid,
};
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
        let mut ulid_generator = Ulid::new();

        for item in sorted_items {
            let rec_adapter = comparer.references.get(&item.id);
            if let Some(rec_adapter) = rec_adapter {
                let mut selected_recommendation = rec_adapter.recommendation.clone();
                selected_recommendation.update(item.result, Self::get_url("domain", 0));

                query_values.push(
                    Self::create_query_value(
                        request,
                        &comparer,
                        &selected_recommendation,
                        rec_adapter,
                        item.result,
                        "Cosine".to_string(),
                        &mut ulid_generator,
                    )
                    .await,
                );
                result.push(selected_recommendation);
            }
        }
        println!("query_values: {:?}", query_values);
        let query = Self::create_query(&query_values).await;
        let _ = RecommendationResponse::save_recommendations(&query).await?;

        Ok(result)
    }

    async fn create_query(query_values: &Vec<String>) -> String {
        let columns = "request_id,customer_id,main_item_id,entity_id,score,request_type,main_item_entity,entity,image,title,resume,algorithm,url,created_at,ulid";
        Orm::insert(&RecommendationResponse::table().await)
            .set_columns(columns)
            .add_many(&query_values.join(","))
            .ready()
    }

    async fn create_query_value(
        request: &RecommendationRequest,
        comparer: &RecommendationComparer,
        selected_recommendation: &Recommendation,
        rec_adapter: &RecommendationAdapter,
        item_result: f32,
        algorithm: String,
        ulid_generator: &mut Ulid,
    ) -> String {
        format!(
            "({},{},{},{},{},'{}','{}','{}','{}','{}','{}','{}','{}','{}','{}')",
            request.request_id,
            request.customer.id,
            comparer.main.item.id,
            selected_recommendation.id,
            item_result,
            request.request_type,
            comparer.main.entity,
            rec_adapter.entity,
            selected_recommendation.image,
            selected_recommendation.title,
            selected_recommendation.resume,
            algorithm,
            selected_recommendation.url,
            get_current_time(),
            ulid_generator
                .generate(
                    &request.request_id,
                    &request.customer.id,
                    &comparer.main.item.id,
                    &selected_recommendation.id
                )
                .await
        )
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
