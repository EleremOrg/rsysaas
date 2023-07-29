use crate::data::{errors::CRUDError, interfaces::db::Manager};

use super::recommendations::Recommendation;
use axum::async_trait;
use rec_rsys::models::Item;

pub struct RecommendationComparer {
    pub main: RecommendationAdapter,
    pub references: Vec<RecommendationAdapter>,
}

impl RecommendationComparer {
    pub async fn new(
        main: RecommendationAdapter,
        references: Vec<RecommendationAdapter>,
    ) -> RecommendationComparer {
        RecommendationComparer { main, references }
    }

    pub async fn get_items_references(&mut self) -> Vec<Item> {
        self.references
            .sort_by(|a, b| a.recommendation.id.cmp(&b.recommendation.id));
        self.references.iter().map(|r| r.item.clone()).collect()
    }

    pub async fn get_final_items(&mut self, final_items: Vec<Item>) -> Vec<RecommendationAdapter> {
        let mut result = Vec::new();
        for item in final_items {
            let adapter = &self.references[(item.id + 1) as usize];
            adapter.item = item;
            adapter.recommendation.score = item.result;
            result.push(adapter.clone());
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct RecommendationAdapter {
    pub item: Item,
    pub recommendation: Recommendation,
    pub entity: String,
}

#[async_trait]
pub trait RecommendationInterface {
    type Model: RecommendationInterface + for<'a> Manager<'a>;

    async fn new_adapter(
        entity: String,
        item: Item,
        id: u32,
        title: String,
        image: String,
        resume: String,
    ) -> RecommendationAdapter {
        RecommendationAdapter {
            item,
            recommendation: Recommendation::default(id, title, image, resume).await,
            entity,
        }
    }

    async fn to_adapter(&self) -> RecommendationAdapter;

    async fn get_adapters(id: u32) -> Result<RecommendationComparer, CRUDError> {
        let instance = <Self::Model as Manager>::get(id).await?;
        let raw_references = instance.get_references_query().await?;
        let mut references: Vec<RecommendationAdapter> = vec![];
        for reference in raw_references {
            references.push(reference.to_adapter().await);
        }
        Ok(RecommendationComparer::new(instance.to_adapter().await, references).await)
    }

    async fn get_references_query(&self) -> Result<Vec<Self::Model>, CRUDError>;
}
