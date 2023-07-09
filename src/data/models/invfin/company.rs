use crate::data::facades::db::Manager;
use crate::web::facade::View;
use axum::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, sqlx::FromRow, Deserialize, Serialize, Default)]

pub struct Company {
    pub id: u32,
    pub ticker: String,
    pub sector: String,
    pub industry: String,
    pub exchange: String,
    pub country: String,
    pub adj: String,
    pub growth: f32,
}

#[async_trait]
impl Manager<'_> for Company {
    async fn table() -> String {
        "companies".to_string()
    }
}
#[async_trait]
impl View<'_> for Company {}
