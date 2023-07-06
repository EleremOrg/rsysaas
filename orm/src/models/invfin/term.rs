use crate::manager::Manager;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize)]

pub struct Term {
    pub id: u32,
    pub title: String,
    pub slug: String,
    pub category: String,
    pub tags: String,
}
#[async_trait]
impl Manager<'_> for Term {
    type Item = Self;

    async fn table() -> String {
        "terms".to_string()
    }
}
