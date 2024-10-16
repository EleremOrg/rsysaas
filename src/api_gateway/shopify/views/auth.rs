use axum::{
    http::HeaderValue,
    response::{IntoResponse, Redirect, Response},
};

use menva::get_env;

use stefn::{AppError, AppState};

use crate::api_gateway::shopify::{
    controllers::auth::{
        create_customer, elegible_to_redirect_to_ouath_flow, elegible_to_redirect_to_profile,
        find_customer_from_shopify, get_auth_token, update_profile, validate_hmac, validate_shop,
    },
    models::auth::{
        ShopifyAccessTokenPayload, ShopifyInitialValidationQuery, ShopifyRedirectAuthQuery,
    },
};

const SHOPIFY_CLIENT_ID: &str = "adb12b988310f4121054c09996645143";
const REDIRECT_URI: &str = "https://ac1f-90-9-172-56.ngrok-free.app/api/v1/shopify/auth/callback";

pub async fn handle_initial_verification(
    state: AppState,
    query: ShopifyInitialValidationQuery,
) -> Result<Response, AppError> {
    let secret = get_env("SHOPIFY_SECRET");
    if !validate_hmac(&query, &secret) {
        return Err(AppError::custom_bad_request(
            "Problem with shopify verification",
        ));
    }

    let redirect = find_customer_from_shopify(&state, &query.shop)
        .await?
        .and_then(elegible_to_redirect_to_profile)
        .unwrap_or(elegible_to_redirect_to_ouath_flow(
            &query.shop,
            SHOPIFY_CLIENT_ID,
            REDIRECT_URI,
        ));
    let mut response = Redirect::to(&redirect).into_response();
    //TODO: not sure that this is working
    response
        .headers_mut()
        .insert("Custom-Auth", HeaderValue::from_str("nonce").unwrap());

    Ok(response)
}

pub async fn handle_authentication(
    state: AppState,
    query: ShopifyRedirectAuthQuery,
) -> Result<Redirect, AppError> {
    let secret = get_env("SHOPIFY_SECRET");
    if !(query.state.eq("nonce") && validate_shop(&query.shop) && validate_hmac(&query, &secret)) {
        return Err(AppError::RoleError);
    };
    let client = reqwest::Client::new();
    let token = get_auth_token(
        &client,
        &ShopifyAccessTokenPayload::new(SHOPIFY_CLIENT_ID, &secret, &query.code),
        &query.shop,
    )
    .await?;

    let redirect = match find_customer_from_shopify(&state, &query.shop).await? {
        Some(profile) => update_profile(&state, &token, profile.pk).await?,
        None => create_customer(&state, &token, &query.shop).await?,
    };

    Ok(Redirect::to(&format!(
        "/{redirect}?token={}",
        token.access_token
    )))
}
