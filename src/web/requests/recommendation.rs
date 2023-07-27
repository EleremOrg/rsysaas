use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    response::Response,
    RequestPartsExt,
};
use hyper::http::request::Parts;
use serde::{Deserialize, Serialize};
use tracing::error;

use std::sync::Arc;

use crate::{
    business::{
        interface::CustomerInterface,
        requests::{RecommendationRequest, RecommendationTarget},
    },
    web::responses::wrong_query,
};

#[async_trait]
pub trait QueryRequest<'a> {
    async fn get_fields_and_values(&self) -> (String, String);

    async fn final_request(
        &self,
        customer: &CustomerInterface,
    ) -> Result<RecommendationRequest, ()>;

    async fn to_generic_request(
        &self,
        customer: &CustomerInterface,
        target: RecommendationTarget,
    ) -> RecommendationRequest;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIRecommendationRequest {
    pub entity: Arc<String>,
    pub target: Arc<String>,
    pub user_id: Option<u32>,
    pub prod_id: Option<u32>,
    pub number_recommendations: Option<u8>,
}

#[async_trait]
impl<'a> QueryRequest<'a> for APIRecommendationRequest {
    async fn get_fields_and_values(&self) -> (String, String) {
        let mut fields = String::from("");
        let mut values = String::from("");
        let parameters = match serde_json::to_value(&self) {
            Ok(obj) => obj,
            _ => panic!("Unexpected JSON value"),
        };
        let obj = match parameters.as_object() {
            Some(val) => val,
            None => panic!("Unexpected JSON value"),
        };
        for (key, value) in obj {
            fields.push_str(format!("{fields}, {key}").as_str());
            values.push_str(format!("{values}, {value}").as_str());
        }
        (fields, values)
    }

    async fn final_request(
        &self,
        customer: &CustomerInterface,
    ) -> Result<RecommendationRequest, ()> {
        match RecommendationTarget::get(&self.target).await {
            Ok(target) => Ok(self.to_generic_request(customer, target).await),
            Err(_) => Err(()),
        }
    }

    async fn to_generic_request(
        &self,
        customer: &CustomerInterface,
        target: RecommendationTarget,
    ) -> RecommendationRequest {
        RecommendationRequest {
            prod_id: self.prod_id,
            user_id: self.user_id,
            number_recommendations: self.number_recommendations.unwrap_or(5),
            entity: self.entity.clone(),
            customer: customer.clone(),
            target: target,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for APIRecommendationRequest
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match parts.extract::<Query<Self>>().await {
            Ok(params) => Ok(params.0),
            Err(err) => return Err(wrong_query(&clean_error_message(err.body_text()).await)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmbedRecommendationRequest {
    pub entity: Arc<String>,
    pub target: Arc<String>,
    pub user_id: Option<u32>,
    pub prod_id: Option<u32>,
    pub number_recommendations: Option<u8>,

    pub title: Arc<String>,
    pub show_image: bool,
    pub show_resume: bool,
    pub orientation: Arc<String>,
    pub is_transparent: bool,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub locale: Arc<String>,
    pub color_theme: Arc<String>,
    pub public_key: Arc<String>,
    pub location_href: Arc<String>,
    pub base_uri: Arc<String>,
    pub doc_url: Arc<String>,
    pub user_agent: Arc<String>,
    pub language: Arc<String>,
    pub languages: Arc<String>,
    pub screen_width: Option<u32>,
    pub screen_height: Option<u32>,
    pub referrer: Arc<String>,
    pub document_title: Arc<String>,
    pub host: Arc<String>,
    pub location: Arc<String>,
}

#[async_trait]
impl<'a> QueryRequest<'a> for EmbedRecommendationRequest {
    async fn get_fields_and_values(&self) -> (String, String) {
        let mut fields = String::from("");
        let mut values = String::from("");
        let parameters = match serde_json::to_value(&self) {
            Ok(obj) => obj,
            Err(err) => {
                error!("Unexpected error {}", err);
                return (fields, values);
            }
        };
        let obj = match parameters.as_object() {
            Some(val) => val,
            None => {
                error!("Unexpected JSON value");
                return (fields, values);
            }
        };
        for (key, value) in obj {
            fields.push_str(format!("{fields}, {key}").as_str());
            values.push_str(format!("{values}, {value}").as_str());
        }
        (fields, values)
    }

    async fn final_request(
        &self,
        customer: &CustomerInterface,
    ) -> Result<RecommendationRequest, ()> {
        match RecommendationTarget::get(&self.target).await {
            Ok(target) => Ok(self.to_generic_request(customer, target).await),
            Err(_) => Err(()),
        }
    }

    async fn to_generic_request(
        &self,
        customer: &CustomerInterface,
        target: RecommendationTarget,
    ) -> RecommendationRequest {
        RecommendationRequest {
            prod_id: self.prod_id,
            user_id: self.user_id,
            number_recommendations: self.number_recommendations.unwrap_or(5),
            entity: self.entity.clone(),
            customer: customer.clone(),
            target: target,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for EmbedRecommendationRequest
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match parts.extract::<Query<Self>>().await {
            Ok(params) => Ok(params.0),
            Err(err) => return Err(wrong_query(&clean_error_message(err.body_text()).await)),
        }
    }
}

async fn clean_error_message(msg: String) -> String {
    if msg.starts_with("Failed to deserialize query string: missing field ") {
        return msg.replace("Failed to deserialize query string: ", "");
    }
    msg
}
