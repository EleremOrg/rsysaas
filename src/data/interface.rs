use crate::data::{
    facades::db::Manager,
    models::invfin::{company::Company, term::Term},
};
use rec_rsys::models::{AsyncItemAdapter, Item};
use std::sync::Arc;

pub async fn get_model_items(prod_id: u32, entity: Arc<String>) -> (Item, Vec<Item>) {
    let model = match entity.as_ref().as_str() {
        "companies" => Company::get(prod_id).await,
        // "terms" => Term::default(),
        _ => Company::get(prod_id).await,
    };
    match model {
        Ok(instance) => (instance.to_item().await, instance.get_references().await),
        Err(_) => (Item::new(0, vec![0.0], Some(0.0)), vec![]),
    }
}
