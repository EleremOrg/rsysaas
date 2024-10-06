use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};

use stefn::{AppError, AppResult, AppState};

pub async fn get_products(
    client: &reqwest::Client,
    access_token: &str,
    shop_url: &str,
) -> Result<serde_json::Value, AppError> {
    let graphql_query = r#"
    {
        products(first: 100) {
            edges {
                node {
                    id
                    title
                    descriptionHtml
                    handle
                    createdAt
                    updatedAt
                    productType
                    tags
                    vendor
                    variants(first: 10) {
                        edges {
                            node {
                                id
                                title
                                price
                                sku
                                availableForSale
                            }
                        }
                    }
                }
            }
        }
    }
    "#;
    client
        .post(&format!(
            "https://{}/admin/api/2024-07/graphql.json",
            shop_url
        ))
        .header("X-Shopify-Access-Token", access_token)
        .json(&json!({
            "query": graphql_query
        }))
        .send()
        .await
        .map_err(|err| AppError::custom_internal(&err.to_string()))? //TODO: only log the errors do not do that
        .json()
        .await
        .map_err(|err| AppError::custom_internal(&err.to_string()))
}
