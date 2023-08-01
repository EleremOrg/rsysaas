use crate::data::{errors::CRUDError, interfaces::db::Manager, models::customer::Customer};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc};

use super::{recommendations::Recommendation, requests::RecommendationRequest};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerFacade {
    pub id: u32,
    pub domain: Arc<String>,
    pub models_related: HashSet<String>,
}

impl CustomerFacade {
    pub async fn get_by_token(token: &str) -> Result<CustomerFacade, CRUDError> {
        Ok(Self::customer_to_interface(Customer::get_by_token(token).await?).await)
    }

    pub async fn get_by_public_token_and_domain(
        token: &str,
        domain: Arc<String>,
    ) -> Result<CustomerFacade, CRUDError> {
        Ok(Self::customer_to_interface(
            Customer::get_by_public_token_and_domain(token, domain).await?,
        )
        .await)
    }

    async fn customer_to_interface(customer: Customer) -> Self {
        CustomerFacade {
            id: customer.id,
            domain: Arc::new(customer.domain),
            models_related: customer
                .models_related
                .split(",")
                .map(|s| s.to_string())
                .collect(),
        }
    }

    pub async fn is_allowed(entity: Arc<String>, token: &str) -> bool {
        let condition = format!("token = '{token}' AND models_related LIKE '%{entity}%'");
        match Customer::exists(&condition).await {
            Ok(exists) => exists,
            Err(_err) => {
                return false;
            }
        }
    }
}
