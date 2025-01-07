use sqlx::SqliteConnection;
use stefn::{AppError, Database};

use crate::api_gateway::shopify::auth::dtos::{
    entities::ShopifyProfile,
    graphql::{Shop, ShopifyAccessTokenResponse},
};

pub async fn update_profile(
    database: &Database,
    token: &ShopifyAccessTokenResponse,
    pk: i64,
) -> Result<String, AppError> {
    let mut tx = database
        .get_connection()
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

pub async fn create_shop(
    shop: &Shop,
    token: &ShopifyAccessTokenResponse,
    con: &mut SqliteConnection,
) -> Result<(), AppError> {
    let shopify_shop_pk =
        sqlx::query("INSERT INTO shopify_shops(id, shop, name, unit_system, weight_unit) VALUES ($1, $2, $3, $4, $5)")
            .bind(&shop.id)
            .bind(&shop.url)
            .bind(&shop.name)
            .bind(&shop.unit_system)
            .bind(&shop.weight_unit)
            .execute(&mut *con)
            .await
            .map_err(|e| AppError::custom_internal(&e.to_string()))?
            .last_insert_rowid();

    sqlx::query(
        "INSERT INTO shopify_shop_tokens(shopify_shop_pk, token, scope) VALUES ($1, $2, $3)",
    )
    .bind(shopify_shop_pk)
    .bind(&token.access_token)
    .bind(&token.scope)
    .execute(&mut *con)
    .await
    .map_err(|e| AppError::custom_internal(&e.to_string()))?;
    Ok(())
}

pub async fn find_customer_from_shopify(
    database: &Database,
    shop: &str,
) -> Result<Option<ShopifyProfile>, AppError> {
    sqlx::query_as(
        r#"select pk, token, scopes, created_at from "shopify_profiles" where shop = $1"#,
    )
    .bind(shop)
    .fetch_optional(database.get_connection())
    .await
    .map_err(|e| AppError::custom_internal(&e.to_string()))
}
