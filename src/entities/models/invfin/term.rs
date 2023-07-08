use crate::entities::facades::{db::Manager, view::View};
use axum::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize, Default)]

pub struct Term {
    pub id: u32,
    pub title: String,
    pub slug: String,
    pub category: String,
    pub tags: String,
}
#[async_trait]
impl Manager<'_> for Term {
    async fn table() -> String {
        "terms".to_string()
    }
}
#[async_trait]
impl View<'_> for Term {}
