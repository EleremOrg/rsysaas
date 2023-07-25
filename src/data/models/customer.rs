use std::sync::Arc;

use crate::data::{errors::CRUDError, facades::db::Manager};
use crate::web::facade::View;
use axum::async_trait;
use rec_rsys::models::{AsyncItemAdapter, Item};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize, Default)]
pub struct PotentialCustomer {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub message: String,
}
#[async_trait]
impl Manager<'_> for PotentialCustomer {
    async fn table() -> String {
        "potential_customers".to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize, Default)]
pub struct Customer {
    pub id: u32,
    pub name: String,
    pub domain: String,
    pub token: String,
    pub public_token: String,
    pub models_related: String,
}
#[async_trait]
impl Manager<'_> for Customer {
    async fn table() -> String {
        "customers".to_string()
    }
}

#[async_trait]
impl View<'_> for Customer {}

#[async_trait]
impl AsyncItemAdapter for Customer {
    async fn to_item(&self) -> Item {
        Item::new(self.id, self.create_values().await, None)
    }
    async fn create_values(&self) -> Vec<f32> {
        let mut values = vec![0.0];
        // [].iter().for_each(|f| values.extend(f));
        values
    }

    async fn get_references(&self) -> Vec<Item> {
        vec![self.to_item().await]
    }
}

impl Customer {
    pub async fn get_by_public_token_and_domain(
        token: Arc<String>,
        domain: Arc<String>,
    ) -> Result<Self, CRUDError> {
        Self::execute_query(
            format!(
                "SELECT * FROM {} WHERE public_token = '{token}' AND domain = '{domain}'",
                Self::table().await
            ),
            Self::connect().await,
        )
        .await
    }

    pub async fn get_by_token(token: &str) -> Result<Self, CRUDError> {
        Self::execute_query(
            format!(
                "SELECT * FROM {} WHERE token = '{token}'",
                Self::table().await
            ),
            Self::connect().await,
        )
        .await
    }

    pub async fn get_items() -> Result<Vec<Item>, CRUDError> {
        Ok(vec![Item::new(1, vec![0.], Some(0.))])
    }
}