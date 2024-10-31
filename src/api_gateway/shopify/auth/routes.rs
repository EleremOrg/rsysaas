use axum::{
    extract::State,
    http::HeaderValue,
    response::{IntoResponse, Redirect, Response},
};

use menva::get_env;

use stefn::{APIState, AppError, Database};

use super::{
    applications::{get_redirect_for_authentication, get_redirect_for_inital_validation},
    dtos::routes::{ShopifyInitialValidationQuery, ShopifyRedirectAuthQuery},
};

pub async fn handle_initial_verification(
    database: State<Database>,
    query: ShopifyInitialValidationQuery,
) -> Result<Response, AppError> {
    let secret = get_env("SHOPIFY_SECRET");

    let redirect = get_redirect_for_inital_validation(query, secret, &database).await?;
    let mut response = Redirect::to(&redirect).into_response();
    //TODO: not sure that this is working
    response
        .headers_mut()
        .insert("Custom-Auth", HeaderValue::from_str("nonce").unwrap());

    Ok(response)
}

pub async fn handle_authentication(
    database: State<Database>,
    query: ShopifyRedirectAuthQuery,
) -> Result<Redirect, AppError> {
    let secret = get_env("SHOPIFY_SECRET");

    get_redirect_for_authentication(query, secret, &database)
        .await
        .map(|r| Redirect::to(&r))
}
