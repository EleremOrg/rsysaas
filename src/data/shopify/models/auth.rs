use std::collections::HashMap;

use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
    response::{IntoResponse, Response},
    RequestPartsExt,
};

use serde::{Deserialize, Serialize};
use stefn::AppError;

pub trait ShopifyQueryInterface {
    fn get_query(&self) -> &HashMap<String, String>;
    fn get_hmac(&self) -> &str;
    fn query_to_string(&self) -> String {
        let mut sorted_params: Vec<_> = self.get_query().iter().collect();
        sorted_params.sort_by(|a, b| a.0.cmp(b.0));

        sorted_params
            .into_iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join("&")
    }
}

#[derive(Debug, Deserialize)]
pub struct ShopifyInitialValidationQuery {
    hmac: String,
    pub shop: String,
    query: HashMap<String, String>,
}

impl ShopifyQueryInterface for ShopifyInitialValidationQuery {
    fn get_query(&self) -> &HashMap<String, String> {
        &self.query
    }

    fn get_hmac(&self) -> &str {
        &self.hmac
    }
}

impl ShopifyInitialValidationQuery {
    fn query_to_string(&self) -> String {
        let mut sorted_params: Vec<_> = self.query.iter().collect();
        sorted_params.sort_by(|a, b| a.0.cmp(b.0));

        sorted_params
            .into_iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join("&")
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for ShopifyInitialValidationQuery
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let mut query_params: Query<HashMap<String, String>> =
            parts.extract().await.map_err(IntoResponse::into_response)?;

        let hmac = query_params
            .remove("hmac")
            .ok_or(AppError::DoesNotExist)
            .map_err(IntoResponse::into_response)?
            .to_string();

        let shop = query_params
            .get("shop")
            .ok_or(AppError::DoesNotExist)
            .map_err(IntoResponse::into_response)?
            .to_string();

        Ok(ShopifyInitialValidationQuery {
            hmac,
            shop,
            query: query_params.0,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct ShopifyRedirectAuthQuery {
    hmac: String,
    pub code: String, //authorization_code
    host: String,     //base64_encoded_hostname
    pub shop: String,
    pub state: String,
    query: HashMap<String, String>,
}

impl ShopifyQueryInterface for ShopifyRedirectAuthQuery {
    fn get_query(&self) -> &HashMap<String, String> {
        &self.query
    }
    fn get_hmac(&self) -> &str {
        &self.hmac
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for ShopifyRedirectAuthQuery
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let mut query_params: Query<HashMap<String, String>> =
            parts.extract().await.map_err(IntoResponse::into_response)?;

        let hmac = query_params
            .remove("hmac")
            .ok_or(AppError::DoesNotExist)
            .map_err(IntoResponse::into_response)?
            .to_string();

        let code = query_params
            .get("code")
            .ok_or(AppError::DoesNotExist)
            .map_err(IntoResponse::into_response)?
            .to_string();

        let host = query_params
            .get("host")
            .ok_or(AppError::DoesNotExist)
            .map_err(IntoResponse::into_response)?
            .to_string();

        let shop = query_params
            .get("shop")
            .ok_or(AppError::DoesNotExist)
            .map_err(IntoResponse::into_response)?
            .to_string();

        let state = query_params
            .get("state")
            .ok_or(AppError::DoesNotExist)
            .map_err(IntoResponse::into_response)?
            .to_string();

        Ok(ShopifyRedirectAuthQuery {
            hmac,
            code,
            host,
            shop,
            state,
            query: query_params.0,
        })
    }
}

/// https://shopify.dev/docs/apps/build/authentication-authorization/access-tokens/offline-access-tokens
#[derive(Debug, Serialize)]
pub struct ShopifyAccessTokenPayload<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    code: &'a str,
}

impl<'a> ShopifyAccessTokenPayload<'a> {
    pub fn new(client_id: &'a str, client_secret: &'a str, code: &'a str) -> Self {
        Self {
            client_id,
            client_secret,
            code,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ShopifyAccessTokenResponse {
    pub access_token: String,
    pub scope: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ShopifyProfile {
    pk: String,
    shop: String,
    token: String,
    created_at: String,
    pub scopes: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_to_string() {
        let mut query = HashMap::new();
        query.insert("acode".to_string(), "first".to_string());
        query.insert(
            "code".to_string(),
            "0907a61c0c8d55e99db179b68161bc00".to_string(),
        );
        query.insert("shop".to_string(), "test-shop.myshopify.com".to_string());
        query.insert("timestamp".to_string(), "1337178173".to_string());

        let query = ShopifyInitialValidationQuery {
            hmac: "700e2dadb827fcc8609e9d5ce208b2e9cdaab9df07390d2cbca10d7c328fc4bf".to_string(),
            shop: "test-shop.myshopify.com".to_string(),
            query,
        };

        assert_eq!(
            query.query_to_string(),
            "acode=first&code=0907a61c0c8d55e99db179b68161bc00&shop=test-shop.myshopify.com&timestamp=1337178173"
        );
    }
}
