use crate::data::interfaces::db::Manager;
use crate::web::interface::View;
use axum::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, sqlx::FromRow, Deserialize, Serialize, Default)]
pub struct RecommendationResponse {
    pub id: u32,
    pub request_id: u32,
    pub customer_id: u32,
    pub prod_id: u32,
    pub entity: String,
    pub image: String,
    pub title: String,
    pub resume: String,
    pub score: f32,
    pub algorithm: String,
    pub url: String,
    pub created_at: String,
}

#[async_trait]
impl Manager<'_> for RecommendationResponse {
    async fn table() -> String {
        "recommendations_responses".to_string()
    }
}

#[async_trait]
impl View<'_> for RecommendationResponse {}
