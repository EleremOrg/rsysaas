use std::sync::Arc;

use crate::data::{errors::CRUDError, interfaces::db::Manager};
use axum::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize, Default)]
pub struct Token {
    pub id: u32,
    pub token: String,
    pub is_active: bool,
    pub is_public: bool,
    pub created_at: String,
    pub updated_at: String,
}
#[async_trait]
impl Manager<'_> for Token {
    async fn table() -> String {
        "tokens".to_string()
    }
}

impl Customer {
    pub async fn check_unique_key(token: &str) -> Result<bool, CRUDError> {
        Self::exists(&format!("is_active = true AND token = '{token}'")).await
    }
}
