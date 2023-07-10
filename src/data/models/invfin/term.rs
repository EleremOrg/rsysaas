use crate::data::facades::db::Manager;
use crate::web::facade::View;
use axum::async_trait;
use rec_rsys::models::{one_hot_encode, sum_encoding_vectors, Item, ItemAdapter};
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

impl ItemAdapter for Term {
    fn to_item(&self) -> Item {
        Item::new(self.id, self.create_values(), None)
    }
    fn create_values(&self) -> Vec<f32> {
        let mut values = vec![0.0];
        // [].iter().for_each(|f| values.extend(f));
        values
    }

    fn get_references(&self) -> Vec<Item> {
        vec![self.to_item()]
    }
}
