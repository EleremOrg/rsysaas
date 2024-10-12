use serde_json::json;

use stefn::AppError;

const BULK_PRODUCTS: &str = r#"
mutation {
  bulkOperationRunQuery(
    query:"""
    {
      products {
        edges {
          node {
            vendor
      status
      priceRangeV2 {
        maxVariantPrice {
          amount
          currencyCode
        }
        minVariantPrice {
          amount
          currencyCode
        }
      }
      category {
        ancestorIds
        childrenIds
        fullName
        id
        isArchived
        isLeaf
        isRoot
        level
        name
        parentId
      }
      createdAt
      id
      productType
      publishedAt
      requiresSellingPlan
      tags
      title
      updatedAt
      feedback {
        summary
      }
      totalInventory
      variantsCount {
        count
        precision
      }
      isGiftCard
      legacyResourceId
      description
      descriptionHtml
          }
        }
      }
    }
    """
  ) {
    bulkOperation {
      id
      status
    }
    userErrors {
      field
      message
    }
  }
}
"#;

const STORE_INFO: &str = r#"{
  shop {
    name
    email
    id
    url
    contactEmail
    createdAt
    currencyCode
    unitSystem
    weightUnit
    shopOwnerName
    billingAddress {
      id
      company
      country
      countryCodeV2
      formattedArea
      address1
      address2
      zip
      provinceCode
      province
    }
    timezoneAbbreviation
    timezoneOffset
    description
  }
}"#;

async fn import_products() {
    let token = "shpua_7799bec5e18408d4f0ca0f78d52ee051";
    // let prods = get_products(client, access_token, shop_url);
}

async fn get_products(
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
