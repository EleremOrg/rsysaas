use std::{collections::HashMap, marker::PhantomData};

use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
    response::{IntoResponse, Response},
    RequestPartsExt,
};

use hmac::{Hmac, Mac};

use regex::Regex;
use sha2::Sha256;

use sqlx::Acquire;
use stefn::{AppError, AppState};

use crate::{
    api_gateway::shopify::models::auth::{
        ShopifyAccessTokenPayload, ShopifyAccessTokenResponse, ShopifyProfile,
        ShopifyQueryInterface, ShopifyRedirectAuthQuery,
    },
    utils::post_request,
};

type HmacSha256 = Hmac<Sha256>;

pub fn elegible_to_redirect_to_ouath_flow(
    shop: &str,
    shopify_client_id: &str,
    redirect_uri: &str,
) -> String {
    format!("https://{shop}/admin/oauth/authorize?client_id={client_id}&scope={scopes}&redirect_uri={redirect_uri}&state={nonce}&grant_options[]={access_mode}",
    shop = shop,
    client_id = shopify_client_id,
    scopes = "",
    redirect_uri = redirect_uri,
    nonce = "nonce",
    access_mode = "per-user")
}

pub fn elegible_to_redirect_to_profile(customer: ShopifyProfile) -> Option<String> {
    match customer.scopes.eq("") {
        true => Some("/login".into()),
        false => None,
    }
}

pub fn validate_hmac<Q: ShopifyQueryInterface>(query: &Q, secret: &str) -> bool {
    calculate_hmac(secret, &query.query_to_string()).eq(query.get_hmac())
}

pub fn validate_shop(shop: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9\-]*\.myshopify\.com$").unwrap();
    re.is_match(&shop)
}

pub async fn update_profile(
    state: &AppState,
    token: &ShopifyAccessTokenResponse,
    pk: i64,
) -> Result<String, AppError> {
    let mut tx = state
        .primary_database
        .begin()
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?;

    let _ = sqlx::query("UPDATE shopify_profiles SET token = $1, scope = $2 WHERE pk = $3)")
        .bind(&token.access_token)
        .bind(&token.scope)
        .bind(pk)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?;

    let _ = tx
        .commit()
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?;
    Ok("login".into())
}

pub async fn create_customer(
    state: &AppState,
    token: &ShopifyAccessTokenResponse,
    shop: &str,
) -> Result<String, AppError> {
    let mut tx = state
        .primary_database
        .begin()
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?;

    let customer_company_pk =
        sqlx::query("INSERT INTO customers_companies(name, domain) VALUES ($1, $2)")
            .bind(shop)
            .bind(shop)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::custom_internal(&e.to_string()))?
            .last_insert_rowid();

    let shopify_profile_pk =
        sqlx::query("INSERT INTO shopify_profiles(shop, token, scope) VALUES ($1, $2, $3)")
            .bind(shop)
            .bind(&token.access_token)
            .bind(&token.scope)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::custom_internal(&e.to_string()))?
            .last_insert_rowid();

    let _ =
            sqlx::query("INSERT INTO shopify_profiles_customers_companies_m2m(shopify_profile_pk, company_pk) VALUES ($1, $2)")
                .bind(&shopify_profile_pk)
                .bind(&customer_company_pk)
                .execute(&mut *tx)
                .await
                .map_err(|e| AppError::custom_internal(&e.to_string()))?
                .last_insert_rowid();

    let _ = tx
        .commit()
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?;

    //TODO: send some info so we can link the user to the shopify app
    Ok("register".into())
}

pub async fn find_customer_from_shopify(
    state: &AppState,
    shop: &str,
) -> Result<Option<ShopifyProfile>, AppError> {
    let mut conn = state
        .primary_database
        .acquire()
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?;
    let conn = conn
        .acquire()
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?;

    sqlx::query_as(
        r#"select pk, token, scopes, created_at from "shopify_profiles" where shop = $1"#,
    )
    .bind(shop)
    .fetch_optional(conn)
    .await
    .map_err(|e| AppError::custom_internal(&e.to_string()))
}

pub async fn get_auth_token<'a>(
    client: &reqwest::Client,
    paylod: &ShopifyAccessTokenPayload<'a>,
    shop: &str,
) -> Result<ShopifyAccessTokenResponse, AppError> {
    let access_token_uri = format!("https://{shop}/admin/oauth/access_token");
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

    #[test]
    fn test_calculate_hmac() {
        let secret = "mysecret";
        let hmac = "60bfb5d37197fa25e128368cc3f0bc6119c26455d7a3fadbbbaba14108825cc2";
        let data = "shop=example.myshopify.com&timestamp=1625151600";

        assert!(calculate_hmac(secret, data).eq(hmac));
    }

    #[test]
    fn test_validate_shop_valid() {
        assert!(validate_shop("valid-shop.myshopify.com"));
    }

    #[test]
    fn test_validate_shop_invalid() {
        assert!(!validate_shop("invalid_shop.com"));
    }
}
