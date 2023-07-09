use crate::{
    data::{errors::CRUDError, facades::db::Manager, models::customer::Customer},
    web::requests::RecommendationQueryRequest,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc};

use super::recommendations::Recommendation;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerInterface {
    pub id: u32,
    pub domain: Arc<String>,
    pub models_related: HashSet<String>,
}

impl CustomerInterface {
    pub async fn get_recommendations(
        &self,
        request: &RecommendationQueryRequest,
    ) -> Result<Vec<Recommendation>, CRUDError> {
        match Recommendation::generate_recommendations(self, request).await {
            Ok(recommendations) => Ok(recommendations),
            Err(err) => Err(err),
        }
    }

    pub async fn get(token: &str) -> Result<CustomerInterface, CRUDError> {
        match Customer::get_by_token(token).await {
            Ok(customer) => Ok(Self::customer_to_interface(customer).await),
            Err(err) => Err(err),
        }
    }
    async fn customer_to_interface(customer: Customer) -> Self {
        CustomerInterface {
            id: customer.id,
            domain: Arc::new(customer.domain),
            models_related: customer
                .models_related
                .split(", ")
                .map(|s| s.to_string())
                .collect(),
        }
    }
    pub async fn is_allowed(entity: Arc<String>, token: &str) -> bool {
        let query = format!("api_key = '{token}' AND models_related LIKE '%{entity}%'");
        match Customer::exists(&query).await {
            Ok(exists) => exists,
            Err(_err) => {
                return false;
            }
        }
    }
}
