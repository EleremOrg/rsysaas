use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, SqliteConnection, Transaction};
use tracing::{error, event, Level};

use aromatic::Orm;

use crate::{
    data::{errors::CRUDError, interfaces::db::Manager},
    web::{interface::View, requests::recommendation::RecommendationRedirect},
};

use super::customer::Customer;

#[derive(Clone, Debug, sqlx::FromRow, Deserialize, Serialize, Default)]
pub struct RecommendationUsed {
    pub id: u32,
    pub created_at: String,
    pub recommendation_response_id: u32,
    pub host: String,
    pub user_agent: String,
}

#[async_trait]
impl Manager<'_> for RecommendationUsed {
    async fn table() -> String {
        "recommendations_used".to_string()
    }
}

impl<'a> RecommendationUsed {
    pub async fn handle_recommendation_usage(
        payload: &RecommendationRedirect,
    ) -> Result<String, CRUDError> {
        let mut transaction = Self::transaction().await?;
        let path_query = Self::get_path_query(payload.uild.as_str()).await;
        let row: Result<(u32, String, String), _> = sqlx::query_as(&path_query)
            .fetch_one(&mut transaction as &mut SqliteConnection)
            .await;

        let (id, domain, entity_path) = match row {
            Ok((id, domain, entity_path)) => (id, domain, entity_path),
            Err(e) => {
                event!(Level::ERROR, "{}", e);
                return Err(CRUDError::NotFound);
            }
        };
        Self::save_recommendation_query(
            transaction,
            &format!("{id},'{}','{}'", payload.host, payload.user_agent),
        )
        .await?;
        Ok(format!("{domain}/{entity_path}"))
    }

    async fn get_path_query(ulid: &str) -> String {
        let responses_table = RecommendationResponse::table().await;
        let customers_table = Customer::table().await;

        Orm::select("r.id, c.domain, r.entity_path")
            .from(&responses_table)
            .as_("r")
            .join(&customers_table)
            .as_("c")
            .on("r.customer_id = c.id")
            .where_()
            .equal("r.ulid", ulid)
            .ready()
    }

    async fn save_recommendation_query(
        mut transaction: Transaction<'a, Sqlite>,
        values: &str,
    ) -> Result<(), CRUDError> {
        let query = Orm::insert(&Self::table().await)
            .set_columns("recommendation_response_id,host,user_agent")
            .add_value(values)
            .ready();

        match sqlx::query(&query)
            .execute(&mut transaction as &mut SqliteConnection)
            .await
        {
            Ok(_row) => {
                Self::commit_transaction(transaction).await?;
                Ok(())
            }
            Err(err) => {
                error!("run create after saved the rec usage: {:?}", err);
                Err(CRUDError::InternalError)
            }
        }
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
    pub entity_path: String,
    pub image: String,
    pub title: String,
    pub resume: String,
    pub score: f32,
    pub algorithm: String,
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

impl RecommendationResponse {
    pub async fn save_recommendations(query: &str) -> Result<u64, CRUDError> {
        let mut transaction = Self::transaction().await?;

        match sqlx::query(&query)
            .execute(&mut transaction as &mut SqliteConnection)
            .await
        {
            Ok(row) => {
                Self::commit_transaction(transaction).await?;
                Ok(row.rows_affected())
            }
            Err(err) => {
                event!(
                    Level::ERROR,
                    function = "save_recommendations",
                    error_message = format!("{err}"),
                    query = query,
                );
                Err(CRUDError::InternalError)
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
    pub target: String,
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
    pub target: String,
    pub user_agent: String,
    pub host: String,
}

#[async_trait]
impl Manager<'_> for APIRecommendationRequestModel {
    async fn table() -> String {
        "api_recommendation_request".to_string()
    }
}
