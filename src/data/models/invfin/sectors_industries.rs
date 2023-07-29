use axum::async_trait;
use serde::{Deserialize, Serialize};

use crate::{data::interfaces::db::Manager, web::interface::View};

#[derive(Clone, Debug, PartialEq, sqlx::FromRow, Deserialize, Serialize, Default)]

pub struct Sector {
    pub id: u32,
    #[sqlx(default)]
    pub name: String,
}

#[async_trait]
impl Manager<'_> for Sector {
    async fn table() -> String {
        "companies_sectors".to_string()
    }
}
#[async_trait]
impl View<'_> for Sector {}

#[derive(Clone, Debug, PartialEq, sqlx::FromRow, Deserialize, Serialize, Default)]
pub struct Industry {
    pub id: u32,
    #[sqlx(default)]
    pub name: String,
}

#[async_trait]
impl Manager<'_> for Industry {
    async fn table() -> String {
        "companies_industries".to_string()
    }
}
#[async_trait]
impl View<'_> for Industry {}
