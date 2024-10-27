use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ShopifyAccessTokenResponse {
    pub access_token: String,
    pub scope: String,
}

#[derive(Debug, Deserialize)]
pub struct StoreInfoResponse {
    pub shop: Shop,
}

#[derive(Debug, Deserialize)]
pub struct Shop {
    pub name: String,
    pub email: String,
    pub id: String,
    pub url: String,
    pub contact_email: String,
    pub currency_code: String,
    pub unit_system: String,
    pub weight_unit: String,
    pub shop_owner_name: String,
    pub billing_address: BillingAddress,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BillingAddress {
    pub company: Option<String>,
    pub country: String,
    pub country_code_v2: String,
}
