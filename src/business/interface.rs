use crate::data::{errors::CRUDError, facades::db::Manager, models::customer::Customer};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc};

use super::{recommendations::Recommendation, requests::RecommendationRequest};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerInterface {
    pub id: u32,
    pub domain: Arc<String>,
    pub models_related: HashSet<String>,
}

impl CustomerInterface {
    pub async fn get_recommendations(
        &self,
        request: &RecommendationRequest,
    ) -> Result<Vec<Recommendation>, CRUDError> {
        match Recommendation::generate_recommendations(request).await {
            Ok(recommendations) => Ok(recommendations),
            Err(err) => Err(err),
        }
    }

    pub async fn get_by_token(token: &str) -> Result<CustomerInterface, CRUDError> {
        match Customer::get_by_token(token).await {
            Ok(customer) => Ok(Self::customer_to_interface(customer).await),
            Err(err) => Err(err),
        }
    }

    pub async fn get_by_public_token_and_domain(
        token: Arc<String>,
        domain: Arc<String>,
    ) -> Result<CustomerInterface, CRUDError> {
        match Customer::get_by_public_token_and_domain(token, domain).await {
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
