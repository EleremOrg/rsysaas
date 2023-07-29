use std::collections::HashMap;

use axum::async_trait;
use rec_rsys::models::Item;

use crate::data::{errors::CRUDError, interfaces::db::Manager};

use super::recommendations::Recommendation;

pub struct RecommendationComparer {
    pub main: RecommendationAdapter,
    pub references: HashMap<u32, RecommendationAdapter>,
}

impl RecommendationComparer {
    pub async fn new(
        main: RecommendationAdapter,
        references: HashMap<u32, RecommendationAdapter>,
    ) -> RecommendationComparer {
        RecommendationComparer { main, references }
    }

    pub async fn get_items_references(&mut self) -> Vec<Item> {
        self.references
            .iter()
            .map(|(_, r)| r.item.clone())
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct RecommendationAdapter {
    pub item: Item,
    pub recommendation: Recommendation,
    pub entity: String,
}

#[async_trait]
pub trait RecommendationInterface
where
    Self: for<'a> Manager<'a>,
{
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
            recommendation: Recommendation::new(id, title, image, resume).await,
            entity,
        }
    }

    async fn to_adapter(&self) -> RecommendationAdapter;

    async fn get_adapters(id: u32) -> Result<RecommendationComparer, CRUDError> {
        let instance = <Self as Manager>::get(id).await?;
        let raw_references = instance.get_references_query().await?;
        let mut references: HashMap<u32, RecommendationAdapter> = HashMap::new();
        for reference in raw_references {
            let adapter = reference.to_adapter().await;
            references.insert(adapter.recommendation.id, adapter);
        }
        Ok(RecommendationComparer::new(instance.to_adapter().await, references).await)
    }

    async fn get_references_query(&self) -> Result<Vec<Self>, CRUDError>;
}
