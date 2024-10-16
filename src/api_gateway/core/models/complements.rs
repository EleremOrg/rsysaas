use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Order {
    id: u64,
    product_id: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Refund {
    id: u64,
    order_id: u64,
    product_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct Client {
    id: u64,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Customer {
    id: u64,
    name: String,
    email: String,
    url: String,
    token: String,
    shopify_token: String,
}
