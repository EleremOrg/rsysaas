use crate::data::facades::db::Manager;
use crate::web::facade::View;
use axum::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize, Default)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub customer_id: u32,
    // #[sqlx(skip)]
}

#[async_trait]
impl Manager<'_> for User {
    async fn table() -> String {
        "users".to_string()
    }
}

#[async_trait]
impl View<'_> for User {}
