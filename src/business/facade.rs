use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use tracing::{event, Level};

use crate::data::{errors::CRUDError, interfaces::db::Manager, models::customer::Customer};

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

    pub async fn create_customer(
        name: &String,
        email: &String,
        domain: &String,
        models_related: &String,
    ) {
        let fields = "name, domain, email, token, public_token, models_related";
        let token = Claims::generate_api_token(&name, &email, &domain, "private").await;
        let public_token = Claims::generate_api_token(&name, &email, &domain, "public").await;
        let values = format!(
            "'{name}', '{domain}', '{email}', '{token}', '{public_token}', '{models_related}'"
        );
        match Customer::create(fields, &values).await {
            Ok(customer) => {
                println!("Private token: {:?}", customer.token);
                println!("Public Token: {:?}", customer.public_token);
            }
            Err(err) => {
                println!("Error creating new customer: {:?}", err);
            }
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    name: String,
    email: String,
    domain: String,
}

impl Claims {
    pub async fn generate_api_token(
        name: &str,
        email: &str,
        domain: &str,
        secret_key: &str,
    ) -> String {
        let header = Header::new(Algorithm::HS256);
        let payload = Claims {
            sub: email.to_owned(),
            name: name.to_owned(),
            email: email.to_owned(),
            domain: domain.to_owned(),
        };

        match encode(
            &header,
            &payload,
            &EncodingKey::from_secret(secret_key.as_bytes()),
        ) {
            Ok(token) => token,
            Err(err) => {
                event!(
                    Level::ERROR,
                    function = "generate_api_token",
                    error = format!("{:?}", err),
                    message = "Could not generate token"
                );
                generate_random_string(name, email, domain).await
            }
        }
    }
}

async fn generate_random_string(name: &str, email: &str, domain: &str) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let input = format!("{}{}{}{}", name, email, domain, timestamp);
    input.hash(&mut hasher);

    format!("{:x}", hasher.finish())
}
