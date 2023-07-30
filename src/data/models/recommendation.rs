use crate::data::{errors::CRUDError, interfaces::db::Manager};
use crate::web::interface::View;
use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::SqliteConnection;
use tracing::error;

#[derive(Clone, Debug, sqlx::FromRow, Deserialize, Serialize, Default)]
pub struct RecommendationUsed {
    pub id: u32,
    pub created_at: String,
    pub recommendation_response_id: u32,
}

#[async_trait]
impl Manager<'_> for RecommendationUsed {
    async fn table() -> String {
        "recommendations_used".to_string()
    }
}

#[async_trait]
impl View<'_> for RecommendationUsed {}

#[derive(Clone, Debug, sqlx::FromRow, Deserialize, Serialize, Default)]
pub struct RecommendationResponse {
    pub id: u32,
    pub ulid: String,
    pub request_id: u32,      // One of the models below
    pub request_type: String, // API or Embed
    pub customer_id: u32,
    pub main_item_id: u32,        // The item from the request
    pub main_item_entity: String, // The entity of the item requested
    pub entity_id: u32,
    pub entity: String,
    pub image: String,
    pub title: String,
    pub resume: String,
    pub score: f32,
    pub algorithm: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[async_trait]
impl Manager<'_> for RecommendationResponse {
    async fn table() -> String {
        "recommendations_responses".to_string()
    }
}

#[async_trait]
impl View<'_> for RecommendationResponse {}

impl RecommendationResponse {
    pub async fn save_recommendations(query: &str) -> Result<u64, CRUDError> {
        let mut transaction = Self::transaction().await?;

        match sqlx::query(&query)
            .execute(&mut transaction as &mut SqliteConnection)
            .await
        {
            Ok(row) => Ok(row.rows_affected()),
            Err(err) => {
                error!("deleting: {:?}", err);
                Err(CRUDError::NotFound)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize, Default)]
pub struct EmbedRecommendationRequestModel {
    pub id: u32,
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
        "embed_recommendation_requests".to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize, Default)]
pub struct APIRecommendationRequestModel {
    pub id: u32,
    pub entity: String,
    pub user_id: Option<u32>,
    pub prod_id: Option<u32>,
    pub number_recommendations: u8,
    pub customer_id: u32,
}

#[async_trait]
impl Manager<'_> for APIRecommendationRequestModel {
    async fn table() -> String {
        "api_recommendation_request".to_string()
    }
}
