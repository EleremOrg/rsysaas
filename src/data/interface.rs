use std::sync::Arc;

use rec_rsys::models::Item;

use super::{
    errors::CRUDError,
    models::{customer::Customer, user::User},
};
use crate::{
    business::interface::{RecommendationComparer, RecommendationInterface},
    data::models::invfin::{company::Company, term::Term},
};

pub async fn get_product_comparer(
    prod_id: u32,
    entity: Arc<String>,
) -> Result<RecommendationComparer, CRUDError> {
    match entity.as_ref().as_str() {
        "companies" => Company::get_adapters(prod_id).await,
        "terms" => Term::get_adapters(prod_id).await,
        _ => Company::get_adapters(prod_id).await,
    }
}

pub async fn get_user_items(user_id: u32) -> Result<(Item, Vec<Item>), CRUDError> {
    User::get_items(user_id).await
}

pub async fn get_generic_items() -> Result<Vec<Item>, CRUDError> {
    Customer::get_items().await
}
