use super::{AppError, AppState};

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, get_current_timestamp, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

const SITE: &str = "elerem.com";

pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct JWTClaims {
    // Registered claims
    iss: String, // Issuer
    sub: String, // Subject
    aud: String, // Audience
    exp: i64,    // Expiration time
    iat: u64,    // Issued at
    jti: String, // JWT ID

    // Private claims
    pub rol: String, // User's role TODO: use an enum
}

impl JWTClaims {
    fn new(user_id: i64, user_role: &str) -> Self {
        JWTClaims {
            iss: SITE.to_owned(),
            sub: user_id.to_string(),
            aud: SITE.to_owned(),
            exp: (Utc::now() + Duration::days(1)).timestamp(),
            iat: get_current_timestamp(),
            jti: String::new(),
            rol: user_role.to_owned(),
        }
    }
}

pub fn create_token(user_id: i64, user_role: &str, state: &AppState) -> Result<String, AppError> {
    encode(
        &Header::default(),
        &JWTClaims::new(user_id, user_role),
        &state.keys.encoding,
    )
    .map_err(AppError::JWTError)
}

#[derive(Clone)]
pub struct JWTUserRequest {
    pub id: i64,
    role: String,
}

impl JWTUserRequest {
    pub fn is_authorized(&self, role: &str) -> bool {
        self.role.eq(role)
    }
}

pub async fn jwt_middleware(
    State(state): State<AppState>,
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let mut validation = Validation::default();
    validation.set_audience(&[SITE]);
    validation.set_issuer(&[SITE]);
    validation.leeway = 60 * 60 * 60 * 24 * 30; //TODO: keep at one hour instead of days
    validation.reject_tokens_expiring_in_less_than = 86400u64;
    validation.set_required_spec_claims(&["iss", "sub", "aud", "exp", "iat", "jti", "rol"]);

    let token_data = decode::<JWTClaims>(bearer.token(), &state.keys.decoding, &validation)
        .map_err(AppError::JWTError)?;

    request.extensions_mut().insert(JWTUserRequest {
        id: token_data
            .claims
            .sub
            .parse::<i64>()
            .map_err(AppError::JWTModified)?,
        role: token_data.claims.rol,
    });

    Ok(next.run(request).await)
}
