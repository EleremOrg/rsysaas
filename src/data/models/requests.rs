use crate::data::interfaces::db::Manager;
use axum::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize, Default)]
pub struct EmbedRecommendationRequestModel {
    pub orientation: String,
    pub entity: String,
    pub title: String,
    pub show_image: bool,
    pub show_resume: bool,
    pub user_id: Option<u32>,
    pub prod_id: Option<u32>,
    pub number_recommendations: Option<u8>,
    pub is_transparent: bool,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub locale: String,
    pub color_theme: String,
    pub public_key: String,
    pub location_href: String,
    pub base_uri: String,
    pub doc_url: String,
    pub user_agent: String,
    pub language: String,
    pub languages: String,
    pub screen_width: Option<u32>,
    pub screen_height: Option<u32>,
    pub referrer: String,
    pub document_title: String,
    pub host: String,
    pub location: String,
    pub customer_id: u32,
}

#[async_trait]
impl Manager<'_> for EmbedRecommendationRequestModel {
    async fn table() -> String {
        "embed_recommendation_request".to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize, Default)]
pub struct APIRecommendationRequestModel {
    pub entity: String,
    pub user_id: Option<u32>,
    pub prod_id: Option<u32>,
    pub number_recommendations: u8,
}

#[async_trait]
impl Manager<'_> for APIRecommendationRequestModel {
    async fn table() -> String {
        "api_recommendation_request".to_string()
    }
}
