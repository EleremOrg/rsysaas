use axum::Extension;
use serde::{Deserialize, Serialize};
use stefn::JWTUserRequest;
use utoipa::{ToResponse, ToSchema};

pub type JWTUser = Extension<JWTUserRequest<PrivateClaims>>;

#[derive(Clone, Deserialize, Serialize)]
pub struct PrivateClaims {
    groups: String,
    comp_pk: i64,
}

impl PrivateClaims {
    pub fn new(groups: String, comp_pk: i64) -> Self {
        Self { groups, comp_pk }
    }
}

#[derive(Clone, Deserialize, Serialize, ToResponse, ToSchema)]
pub struct JWTResponse {
    jwt: String,
}
impl JWTResponse {
    pub fn new(jwt: String) -> Self {
        Self { jwt }
    }
}