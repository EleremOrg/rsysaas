use crate::manager::Manager;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize)]
pub struct Customer {
    pub id: u32,
    pub name: String,
    pub domain: String,
    pub api_key: String,
}
#[async_trait]
impl Manager<'_> for Customer {
    type Item = Self;

    async fn table() -> String {
        "customers".to_string()
    }
}
