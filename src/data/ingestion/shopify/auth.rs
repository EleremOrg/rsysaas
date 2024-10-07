use std::collections::HashMap;

use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::{request::Parts, HeaderValue},
    response::{IntoResponse, Redirect, Response},
    RequestPartsExt,
};

use hmac::{Hmac, Mac};
use menva::get_env;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use stefn::{AppError, AppState};

use crate::utils::post_request;

type HmacSha256 = Hmac<Sha256>;
const SHOPIFY_CLIENT_ID: &str = "adb12b988310f4121054c09996645143";

#[derive(Debug, Deserialize)]
pub struct ShopifyQuery {
    hmac: String,
    query: HashMap<String, String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for ShopifyQuery
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

        Ok(ShopifyQuery {
            hmac,
            query: query_params.0,
        })
    }
}

impl ShopifyQuery {
    fn query_to_string(&self) -> String {
        let mut sorted_params: Vec<_> = self.query.iter().collect();
        sorted_params.sort_by(|a, b| a.0.cmp(b.0));

        sorted_params
            .into_iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join("&")
    }

    fn is_valid(&self, secret: &str) -> bool {
        let calculated_hmac = calculate_hmac(secret, &self.query_to_string());
        calculated_hmac == self.hmac
    }
}

pub async fn handle_initial_verification(params: ShopifyQuery) -> Result<Response, AppError> {
    let secret = get_env("SHOPIFY_SECRET");
    if params.is_valid(&secret) {
        let redirect = format!("https://{shop}/admin/oauth/authorize?client_id={client_id}&scope={scopes}&redirect_uri={redirect_uri}&state={nonce}&grant_options[]={access_mode}",
		shop = "quickstart-f533348c.myshopify.com",
		client_id = SHOPIFY_CLIENT_ID,
		scopes = "",
		redirect_uri = "https://ac1f-90-9-172-56.ngrok-free.app/api/v1/shopify/auth/callback",
		nonce = "nonce",
		access_mode = "per-user");

        let mut response = Redirect::to(&redirect).into_response();
        //TODO: not sure that this is working
        response
            .headers_mut()
            .insert("Custom-Auth", HeaderValue::from_str("nonce").unwrap());

        Ok(response)
    } else {
        Err(AppError::custom_internal(
            "Problem with shopify verification",
        ))
    }
}

#[derive(Debug, Deserialize)]
pub struct ShopifyRedirectAuth {
    hmac: String,
    code: String, //authorization_code
    host: String, //base64_encoded_hostname
    shop: String,
    state: String,
    timestamp: i64,
}

impl ShopifyRedirectAuth {
    fn validate_shop(&self) -> bool {
        let re = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9\-]*\.myshopify\.com$").unwrap();
        re.is_match(&self.shop)
    }

    fn validate_hmac(&self, secret: &str) -> bool {
        let query = format!(
            "code={}&host={}&shop={}&state={}&timestamp={}",
            self.code, self.host, self.shop, self.state, self.timestamp
        );
        let calculated_hmac = calculate_hmac(secret, &query);
        calculated_hmac == self.hmac
    }

    fn validate_state(&self, secret: &str) -> bool {
        self.state == secret
    }
}

pub async fn handle_authentication(
    state: AppState,
    query: Query<ShopifyRedirectAuth>,
) -> Result<Redirect, AppError> {
    let secret = get_env("SHOPIFY_SECRET");
    if !(query.validate_state("nonce") && query.validate_shop() && query.validate_hmac(&secret)) {
        return Err(AppError::RoleError);
    };
    let client = reqwest::Client::new();
    let token = get_auth_token(
        &client,
        &ShopifyAccessTokenPayload::new(SHOPIFY_CLIENT_ID, &secret, &query.code),
        &query.shop,
    )
    .await?;
    //TODO: save and or check if the user exists

    let redirection = format!("/dashboard?token={}", token.access_token);
    Ok(Redirect::to(&redirection))
}

/// https://shopify.dev/docs/apps/build/authentication-authorization/access-tokens/offline-access-tokens
#[derive(Debug, Serialize)]
struct ShopifyAccessTokenPayload<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    code: &'a str,
}

impl<'a> ShopifyAccessTokenPayload<'a> {
    fn new(client_id: &'a str, client_secret: &'a str, code: &'a str) -> Self {
        Self {
            client_id,
            client_secret,
            code,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ShopifyAccessTokenResponse {
    access_token: String,
    scope: String,
}

async fn get_auth_token<'a>(
    client: &reqwest::Client,
    paylod: &ShopifyAccessTokenPayload<'a>,
    shop: &str,
) -> Result<ShopifyAccessTokenResponse, AppError> {
    let access_token_uri = format!("https://{}/admin/oauth/access_token", shop);
    post_request(client, paylod, &access_token_uri).await
}

fn calculate_hmac(secret: &str, data: &str) -> String {
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(data.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_query_to_string() {
        let mut params = HashMap::new();
        params.insert("acode".to_string(), "first".to_string());
        params.insert(
            "code".to_string(),
            "0907a61c0c8d55e99db179b68161bc00".to_string(),
        );
        params.insert("shop".to_string(), "test-shop.myshopify.com".to_string());
        params.insert("timestamp".to_string(), "1337178173".to_string());

        let query = ShopifyQuery {
            hmac: "700e2dadb827fcc8609e9d5ce208b2e9cdaab9df07390d2cbca10d7c328fc4bf".to_string(),
            query: params,
        };

        assert_eq!(
            query.query_to_string(),
            "acode=first&code=0907a61c0c8d55e99db179b68161bc00&shop=test-shop.myshopify.com&timestamp=1337178173"
        );
    }

    #[test]
    fn test_calculate_hmac() {
        let mut params = HashMap::new();
        params.insert(
            "host".to_string(),
            "YWRtaW4uc2hvcGlmeS5jb20vc3RvcmUvcXVpY2tzdGFydC1mNTMzMzQ4Yw".to_string(),
        );
        params.insert(
            "code".to_string(),
            "0907a61c0c8d55e99db179b68161bc00".to_string(),
        );
        params.insert(
            "session".to_string(),
            "897673e5887d095d1546f0a0ba6104f25a5feb1f0e0583dd180513b7a6d36a41".to_string(),
        );
        params.insert(
            "shop".to_string(),
            "quickstart-f533348c.myshopify.com".to_string(),
        );
        params.insert("timestamp".to_string(), "1728212657".to_string());

        let query = ShopifyQuery {
            hmac: "039577ecb416257839a1441eb1e2439d0b35747a45ab08fc4c7769d26b689b2d".to_string(),
            query: params,
        };
        let secret = "16ae889726989eda483058d6ee828b23";

        let result = calculate_hmac(secret, &query.query_to_string());

        assert_eq!(result, query.hmac);
    }

    #[test]
    fn test_is_valid() {
        let mut params = HashMap::new();
        params.insert(
            "host".to_string(),
            "YWRtaW4uc2hvcGlmeS5jb20vc3RvcmUvcXVpY2tzdGFydC1mNTMzMzQ4Yw".to_string(),
        );
        params.insert(
            "code".to_string(),
            "0907a61c0c8d55e99db179b68161bc00".to_string(),
        );
        params.insert(
            "session".to_string(),
            "897673e5887d095d1546f0a0ba6104f25a5feb1f0e0583dd180513b7a6d36a41".to_string(),
        );
        params.insert(
            "shop".to_string(),
            "quickstart-f533348c.myshopify.com".to_string(),
        );
        params.insert("timestamp".to_string(), "1728212657".to_string());
        let secret = "16ae889726989eda483058d6ee828b23";
        let query = ShopifyQuery {
            hmac: "039577ecb416257839a1441eb1e2439d0b35747a45ab08fc4c7769d26b689b2d".to_string(),
            query: params,
        };

        assert!(query.is_valid(secret));
    }
}
