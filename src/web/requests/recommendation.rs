use std::{collections::HashMap, sync::Arc};

use axum::{
    async_trait,
    extract::{FromRequestParts, Path, Query},
    response::{IntoResponse, Response},
    RequestPartsExt, TypedHeader,
};
use headers::{Host, UserAgent};
use hyper::{http::request::Parts, StatusCode};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    business::{
        facade::CustomerFacade,
        requests::{RecommendationRequest, RecommendationTarget},
    },
    web::responses::wrong_query,
};

#[async_trait]
pub trait QueryRequest<'a>
where
    Self: Serialize + Deserialize<'a>,
{
    async fn get_fields_and_values(&self) -> (String, String) {
        let mut fields = String::from("");
        let mut values = String::from("");
        let parameters = match serde_json::to_value(&self) {
            Ok(obj) => obj,
            _ => panic!("Unexpected JSON value"),
        };
        for (key, raw_value) in parameters.as_object().unwrap() {
            if let Some(value) = raw_value.as_str() {
                if !value.is_empty() {
                    fields.push_str(format!("{key},").as_str());
                    match value.parse::<u8>() {
                        Ok(v) => values.push_str(format!("{v},").as_str()),
                        Err(_) => values.push_str(format!("'{value}',").as_str()),
                    }
                };
            }
        }
        fields.pop();
        values.pop();
        (fields, values)
    }

    async fn get_request(
        &self,
        customer: &CustomerFacade,
    ) -> Result<RecommendationRequest, Response>;

    async fn to_generic_request(
        &self,
        customer: &CustomerFacade,
        target: RecommendationTarget,
    ) -> RecommendationRequest;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct APIRecommendationRequest {
    pub entity: Arc<String>,
    pub target: Arc<String>,
    pub user_id: Option<String>,
    pub prod_id: Option<String>,
    pub number_recommendations: Option<String>,
    pub user_agent: Option<String>,
    pub host: Option<String>,
}

impl APIRecommendationRequest {
    async fn add_headers_values(&mut self, user_agent: String, host: String) {
        self.user_agent = Some(user_agent);
        self.host = Some(host);
    }
}

#[async_trait]
impl<'a> QueryRequest<'a> for APIRecommendationRequest {
    async fn get_request(
        &self,
        customer: &CustomerFacade,
    ) -> Result<RecommendationRequest, Response> {
        match RecommendationTarget::get(&self.target).await {
            Ok(target) => Ok(self.to_generic_request(customer, target).await),
            Err(err) => Err(wrong_query(err)),
        }
    }

    async fn to_generic_request(
        &self,
        customer: &CustomerFacade,
        target: RecommendationTarget,
    ) -> RecommendationRequest {
        RecommendationRequest {
            prod_id: correct_value(&self.prod_id).await,
            user_id: correct_value(&self.user_id).await,
            number_recommendations: correct_number(&self.number_recommendations).await,
            entity: self.entity.clone(),
            customer: customer.clone(),
            target,
            request_id: RecommendationRequest::save_api_query(self, customer.id).await,
            request_type: String::from("API"),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for APIRecommendationRequest
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let mut x = match parts.extract::<Query<Self>>().await {
            Ok(params) => params.0,
            Err(err) => {
                error!("APIRecommendationRequest from_request_parts: {:?}", err);
                return Err(wrong_query(&clean_error_message(err.body_text()).await));
            }
        };
        let user_agent = get_user_agent(parts).await?;
        let host = get_host(parts).await?;
        x.add_headers_values(user_agent, host).await;
        Ok(x)
    }
}

async fn get_user_agent(parts: &mut Parts) -> Result<String, Response> {
    match parts
        .extract::<TypedHeader<UserAgent>>()
        .await
        .map(|user_agent| user_agent.as_str().to_owned())
    {
        Ok(user_agent) => Ok(user_agent),
        Err(err) => {
            error!(
                "APIRecommendationRequest from_request_parts get_user_agent: {:?}",
                err
            );
            Err(wrong_query(&err.reason()))
        }
    }
}

async fn get_host(parts: &mut Parts) -> Result<String, Response> {
    match parts
        .extract::<TypedHeader<Host>>()
        .await
        .map(|host: TypedHeader<Host>| host.hostname().to_owned())
    {
        Ok(host) => Ok(host),
        Err(err) => {
            error!(
                "APIRecommendationRequest from_request_parts get_host: {:?}",
                err
            );
            Err(wrong_query(&err.reason()))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct EmbedRecommendationRequest {
    pub entity: Arc<String>,
    pub target: Arc<String>,
    pub user_id: Option<String>,
    pub prod_id: Option<String>,
    pub number_recommendations: Option<String>,

    pub title: Arc<String>,
    pub show_image: bool,
    pub show_resume: bool,
    pub orientation: Arc<String>,
    pub is_transparent: bool,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub locale: Arc<String>,
    pub color_theme: Arc<String>,
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
    async fn get_request(
        &self,
        customer: &CustomerFacade,
    ) -> Result<RecommendationRequest, Response> {
        match RecommendationTarget::get(&self.target).await {
            Ok(target) => Ok(self.to_generic_request(customer, target).await),
            Err(err) => Err(wrong_query(err)),
        }
    }

    async fn to_generic_request(
        &self,
        customer: &CustomerFacade,
        target: RecommendationTarget,
    ) -> RecommendationRequest {
        RecommendationRequest {
            prod_id: correct_value(&self.prod_id).await,
            user_id: correct_value(&self.user_id).await,
            number_recommendations: correct_number(&self.number_recommendations).await,
            entity: self.entity.clone(),
            customer: customer.clone(),
            target,
            request_id: RecommendationRequest::save_embed_query(self, customer.id).await,
            request_type: String::from("Embed"),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for EmbedRecommendationRequest
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
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

async fn correct_value(value: &Option<String>) -> Option<u32> {
    value.clone().map(|s| s.parse::<u32>().ok()).flatten()
}

async fn correct_number(value: &Option<String>) -> u8 {
    value
        .clone()
        .unwrap_or("5".to_string())
        .parse::<u8>()
        .unwrap_or(5)
}

#[derive(Serialize, Deserialize)]
pub struct RecommendationRedirect {
    pub user_agent: Arc<String>,
    pub host: Arc<String>,
    pub uild: Arc<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for RecommendationRedirect
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> =
            parts.extract().await.map_err(IntoResponse::into_response)?;

        let uild = params
            .get("ulid")
            .ok_or_else(|| (StatusCode::NOT_FOUND, "ulid missing").into_response())?;

        let user_agent = get_user_agent(parts).await?;
        let host = get_host(parts).await?;
        Ok(Self {
            user_agent: Arc::new(user_agent),
            host: Arc::new(host),
            uild: Arc::new(uild.to_owned()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_fields_and_values_with_valid_struct() {
        let my_struct = APIRecommendationRequest {
            entity: Arc::new(String::from("entity")),
            target: Arc::new(String::from("target")),
            user_id: Some(String::from("15")),
            prod_id: Some(String::from("2")),
            number_recommendations: Some(String::from("5")),
            user_agent: None,
            host: None,
        };
        let (fields, values) = my_struct.get_fields_and_values().await;
        assert_eq!(
            fields,
            "entity,number_recommendations,prod_id,target,user_id"
        );
        assert_eq!(values, "'entity',5,2,'target',15");
    }

    #[tokio::test]
    async fn test_get_fields_and_values_none_with_valid_struct() {
        let my_struct = APIRecommendationRequest {
            entity: Arc::new(String::from("entity")),
            target: Arc::new(String::from("target")),
            user_id: Some(String::from("")),
            prod_id: None,
            number_recommendations: None,
            user_agent: None,
            host: None,
        };
        let (fields, values) = my_struct.get_fields_and_values().await;
        assert_eq!(fields, "entity,target");
        assert_eq!(values, "'entity','target'");
    }
}
