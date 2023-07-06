use crate::manager::Manager;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub customer_id: u32,
}

#[async_trait]
impl Manager<'_> for User {
    type Item = Self;

    async fn table() -> String {
        "users".to_string()
    }
}
