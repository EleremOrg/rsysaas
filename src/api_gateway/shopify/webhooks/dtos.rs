use serde::Deserialize;

/// https://shopify.dev/docs/api/admin-graphql/2024-10/objects/BulkOperation
#[derive(Debug, Deserialize)]
pub struct BulkOperation {
    _id: String,
    _completed_at: String,
    _created_at: String,
    _error_code: Option<String>,
    _object_count: u64,
    _status: String,
    _type_: String,
    _url: String,
}

#[derive(Debug, Deserialize)]
pub struct AppUninstalledPayload {
    _id: u64,
    _name: String,
    _email: String,
    _domain: Option<String>,
    _province: String,
    _country: String,
    _address1: String,
}
