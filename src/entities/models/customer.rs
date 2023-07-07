use crate::interfaces::db::Manager;
use async_trait::async_trait;
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
