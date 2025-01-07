use hmac::{Hmac, Mac};

use regex::Regex;
use sha2::Sha256;
use stefn::{hash_password, AppError, Database};

use crate::entities::customers;

use super::{
    dtos::{
        entities::{ShopifyAccessTokenPayload, ShopifyProfile},
        routes::{ShopifyInitialValidationQuery, ShopifyQueryInterface, ShopifyRedirectAuthQuery},
    },
    infrastructures::{create_shop, find_customer_from_shopify, update_profile, ShopifyClient},
};

type HmacSha256 = Hmac<Sha256>;

const SHOPIFY_CLIENT_ID: &str = "adb12b988310f4121054c09996645143";
const REDIRECT_URI: &str = "https://ac1f-90-9-172-56.ngrok-free.app/api/v1/shopify/auth/callback";

pub async fn get_redirect_for_authentication(
    query: ShopifyRedirectAuthQuery,
    secret: String,
    database: &Database,
) -> Result<String, AppError> {
    if !(query.state.eq("nonce") && validate_shop(&query.shop) && validate_hmac(&query, &secret)) {
        return Err(AppError::RoleError);
    };

    let client = ShopifyClient::new(&query.shop);
    let access_token_payload =
        ShopifyAccessTokenPayload::new(SHOPIFY_CLIENT_ID, &secret, &query.code);

    match find_customer_from_shopify(database, &query.shop).await? {
        Some(profile) => update_customer(database, &client, &access_token_payload, profile).await,
        None => create_new_customer(database, &client, &access_token_payload).await,
    }
}

async fn create_new_customer<'a>(
    database: &Database,
    client: &ShopifyClient<'a>,
    access_token_payload: &ShopifyAccessTokenPayload<'a>,
) -> Result<String, AppError> {
    let token = client.get_auth_token(access_token_payload).await?;
    let store = client.get_shop_information().await?;
    let tx = database
        .get_connection()
        .begin()
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?;

    let password = hash_password("Change$Fast123")?;

    let mut tx = customers::Builder::new(tx)
        .user_from_password(&password)
        .await?
        .add_email(&store.shop.email)
        .await?
        .add_email(&store.shop.contact_email)
        .await?
        .add_to_admin_group()
        .await?
        .customer_from_names(&store.shop.shop_owner_name, &store.shop.shop_owner_name)
        .await?
        .add_to_new_company(
            &store.shop.name,
            &store.shop.url,
            &store.shop.description.as_ref().unwrap_or(&String::new()),
            &store.shop.billing_address.country_code_v2,
            &store.shop.currency_code,
        )
        .await?
        .release();

    create_shop(&store.shop, &token, &mut *tx).await?;

    tx.commit()
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?;

    let _ = client.request_bulk_products().await;
    // let _ = client.request_bulk_orders().await;
    // let _ = client.request_bulk_returns().await;

    //TODO: send some info so we can link the user to the shopify app
    Ok("register".into())
}

async fn update_customer<'a>(
    database: &Database,
    client: &ShopifyClient<'a>,
    access_token_payload: &ShopifyAccessTokenPayload<'a>,
    profile: ShopifyProfile,
) -> Result<String, AppError> {
    let token = client.get_auth_token(access_token_payload).await?;

    update_profile(database, &token, profile.pk).await
}

pub async fn get_redirect_for_inital_validation(
    query: ShopifyInitialValidationQuery,
    secret: String,
    database: &Database,
) -> Result<String, AppError> {
    if !validate_hmac(&query, &secret) {
        return Err(AppError::custom_bad_request(
            "Problem with shopify verification",
        ));
    }
    //TODO: look at the shopify docs and add better names for the steps
    Ok(find_customer_from_shopify(database, &query.shop)
        .await?
        .and_then(redirect_to_profile)
        .unwrap_or(redirect_to_ouath_flow(
            &query.shop,
            SHOPIFY_CLIENT_ID,
            REDIRECT_URI,
        )))
}

fn redirect_to_ouath_flow(shop: &str, shopify_client_id: &str, redirect_uri: &str) -> String {
    format!("https://{shop}/admin/oauth/authorize?client_id={client_id}&scope={scopes}&redirect_uri={redirect_uri}&state={nonce}&grant_options[]={access_mode}",
    shop = shop,
    client_id = shopify_client_id,
    scopes = "",
    redirect_uri = redirect_uri,
    nonce = "nonce",
    access_mode = "per-user")
}

fn redirect_to_profile(customer: ShopifyProfile) -> Option<String> {
    match customer.scopes.eq("") {
        true => Some("/login".into()),
        false => None,
    }
}

fn validate_hmac<Q: ShopifyQueryInterface>(query: &Q, secret: &str) -> bool {
    calculate_hmac(secret, &query.query_to_string()).eq(query.get_hmac())
}

fn validate_shop(shop: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9\-]*\.myshopify\.com$").unwrap();
    re.is_match(&shop)
}

fn calculate_hmac(secret: &str, data: &str) -> String {
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(data.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_validate_hmac() {
        let secret = "mysecret";
        let mut query = HashMap::new();
        query.insert("shop".to_string(), "example.myshopify.com".to_string());
        query.insert("timestamp".to_string(), "1625151600".to_string());

        let query = ShopifyInitialValidationQuery {
            hmac: "60bfb5d37197fa25e128368cc3f0bc6119c26455d7a3fadbbbaba14108825cc2".to_string(),
            shop: "example.myshopify.com".to_string(),
            query,
        };
        assert!(validate_hmac(&query, secret));
    }

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
