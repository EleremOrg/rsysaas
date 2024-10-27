use crate::{
    api_gateway::shopify::auth::dtos::{
        entities::ShopifyAccessTokenPayload,
        graphql::{ShopifyAccessTokenResponse, StoreInfoResponse},
    },
    utils,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use stefn::AppError;

const STORE_INFO: &str = r#"{
    shop {
      name
      email
      id
      url
      contactEmail
      currencyCode
      unitSystem
      weightUnit
      shopOwnerName
      billingAddress {
        company
        country
        countryCodeV2
      }
      description
    }
  }"#;

pub struct ShopifyClient<'a> {
    client: reqwest::Client,
    pub shop_url: &'a str,
    access_token: Option<&'a str>,
}

impl<'a> ShopifyClient<'a> {
    pub fn new(shop_url: &'a str) -> Self {
        Self {
            client: reqwest::Client::new(),
            shop_url,
            access_token: None,
        }
    }

    pub async fn get_auth_token(
        &self,
        paylod: &ShopifyAccessTokenPayload<'a>,
    ) -> Result<ShopifyAccessTokenResponse, AppError> {
        let access_token_uri = format!("https://{}/admin/oauth/access_token", &self.shop_url);
        utils::post_request(&self.client, paylod, &access_token_uri).await
    }

    pub async fn get_shop_information(&self) -> Result<StoreInfoResponse, AppError> {
        self.request(STORE_INFO).await
    }

    async fn request<T: DeserializeOwned>(&self, query: &str) -> Result<T, AppError> {
        let request = self
            .client
            .post(&format!(
                "https://{}/admin/api/2024-07/graphql.json",
                self.shop_url
            ))
            .header("X-Shopify-Access-Token", self.access_token.unwrap())
            .json(&json!({"query": query}));
        utils::request(request).await
    }
}
