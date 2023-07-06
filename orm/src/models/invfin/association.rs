use crate::manager::Manager;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize)]

pub struct Association {
    pub id: u32,
    pub table_related: String,
    pub row_id: u32,
}
#[async_trait]
impl Manager<'_> for Association {
    type Item = Self;

    async fn table() -> String {
        "associations".to_string()
    }
}
