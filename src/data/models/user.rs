use crate::data::interfaces::db::Manager;
use crate::web::interface::View;
use axum::async_trait;
use rec_rsys::models::{AsyncItemAdapter, Item};
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

#[async_trait]
impl AsyncItemAdapter for User {
    async fn to_item(&self) -> Item {
        Item::new(self.id, self.create_values().await, None)
    }
    async fn create_values(&self) -> Vec<f32> {
        // let mut values = vec![0.0];
        // [].iter().for_each(|f| values.extend(f));
        vec![0.0]
    }

    async fn get_references(&self) -> Vec<Item> {
        vec![self.to_item().await]
    }
}
