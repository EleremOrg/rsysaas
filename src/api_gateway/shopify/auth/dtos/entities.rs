use serde::Serialize;
use sqlx::prelude::FromRow;

use crate::entities::{customers::Builder, products::Customer};

use super::graphql::Shop;

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

#[derive(Debug, FromRow)]
pub struct ShopifyProfile {
    pub pk: i64,
    token: String,
    pub scopes: String,
    created_at: String,
}
