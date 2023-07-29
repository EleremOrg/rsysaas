use std::sync::Arc;

use super::errors::CRUDError;
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
