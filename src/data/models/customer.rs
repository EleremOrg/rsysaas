use crate::data::{errors::CRUDError, facades::db::Manager};
use crate::web::facade::View;
use axum::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize, Default)]
pub struct Customer {
    pub id: u32,
    pub name: String,
    pub domain: String,
    pub api_key: String,
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

impl Customer {
    pub async fn get_by_token(token: &str) -> Result<Self, CRUDError> {
        Self::execute_query(
            format!(
                "SELECT * FROM {} WHERE api_key = '{token}'",
                Self::table().await
            ),
            Self::connect().await,
        )
        .await
    }
}
