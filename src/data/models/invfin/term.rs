use crate::{
    business::interface::{RecommendationAdapter, RecommendationInterface},
    data::{errors::CRUDError, interfaces::db::Manager},
    web::interface::View,
};
use aromatic::Orm;
use axum::async_trait;
use rec_rsys::models::{AsyncItemAdapter, Item};
use serde::{Deserialize, Serialize};



#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize, Default)]

pub struct Term {
    pub id: u32,
    #[sqlx(default)]
    pub title: String,
    #[sqlx(default)]
    pub image: String,
    #[sqlx(default)]
    pub resume: String,
    #[sqlx(default)]
    pub path: String,
    #[sqlx(default)]
    pub category: String,
    #[sqlx(default)]
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

#[async_trait]
impl AsyncItemAdapter for Term {
    async fn to_item(&self) -> Item {
        Item::new(self.id, self.create_values().await, None)
    }
    async fn create_values(&self) -> Vec<f32> {
        vec![22.33]
    }

    async fn get_references(&self) -> Vec<Item> {
        vec![self.to_item().await]
    }
}

#[async_trait]
impl RecommendationInterface for Term {
    async fn to_adapter(&self) -> RecommendationAdapter {
        <Term as RecommendationInterface>::new_adapter(
            Term::table().await,
            self.to_item().await,
            self.id,
            self.title.clone(),
            self.image.clone(),
            self.resume.clone(),
            self.path.clone(),
        )
        .await
    }

    // TODO: take into consideration the fact that a customer may query a table with data from other customers
    async fn get_references_query(&self) -> Result<Vec<Term>, CRUDError> {
        let query = Orm::select("id, path, title, resume, image, tags, category")
            .from(&Self::table().await)
            .where_()
            .not_equal("id", &self.id.to_string())
            .ready();
        Self::rows_to_vec(query, Self::transaction().await?).await
    }
}
