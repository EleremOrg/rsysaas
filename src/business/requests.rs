use std::sync::Arc;

use axum::response::Response;
use serde::{Deserialize, Serialize};

use crate::{
    data::{
        errors::CRUDError,
        interfaces::db::Manager,
        models::recommendation::{APIRecommendationRequestModel, EmbedRecommendationRequestModel},
    },
    web::{
        requests::recommendation::{
            APIRecommendationRequest, EmbedRecommendationRequest, QueryRequest,
        },
        responses::{our_fault, success, wrong_query},
    },
};

use super::facade::CustomerFacade;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecommendationRequest {
    pub prod_id: Option<u32>,
    pub user_id: Option<u32>,
    pub number_recommendations: u8,
    pub entity: Arc<String>,
    pub customer: CustomerFacade,
    pub target: RecommendationTarget,
    pub request_id: u32,
    pub request_type: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum RecommendationTarget {
    User,
    Product,
    Generic,
}

impl RecommendationTarget {
    pub async fn get(target: &str) -> Result<Self, &str> {
        match target {
            "user" => Ok(RecommendationTarget::User),
            "product" => Ok(RecommendationTarget::Product),
            "generic" => Ok(RecommendationTarget::Generic),
            _ => Err("Wrong target, you must chose between user, product or generic"),
        }
    }
}

impl RecommendationRequest {
    pub async fn get_id(&self) -> u32 {
        match self.target {
            RecommendationTarget::Generic => 0,
            RecommendationTarget::User => self.user_id.unwrap(),
            RecommendationTarget::Product => self.prod_id.unwrap(),
        }
    }

    pub async fn recommend(&self) -> Response {
        match self.validate_request().await {
            Ok(_) => self.get_recommendations().await,
            Err(err) => err,
        }
    }

    pub async fn get_recommendations(&self) -> Response {
        match self.customer.get_recommendations(self).await {
            Ok(recs) => success(recs),
            Err(err) => match err {
                CRUDError::NotFound => wrong_query(&format!("wrong id {:?}", self.get_id().await)),
                CRUDError::MaxRetry => our_fault(),
                _ => our_fault(),
            },
        }
    }

    pub async fn validate_request(&self) -> Result<(), Response> {
        match self.target {
            RecommendationTarget::Generic => self.validate_generic_request().await,
            RecommendationTarget::User => self.validate_user_request().await,
            RecommendationTarget::Product => self.validate_product_request().await,
        }
    }

    pub async fn validate_generic_request(&self) -> Result<(), Response> {
        Ok(())
    }

    pub async fn validate_user_request(&self) -> Result<(), Response> {
        if self.entity.is_empty() {
            return Err(wrong_query("entity needed"));
        }
        Ok(())
    }

    pub async fn validate_product_request(&self) -> Result<(), Response> {
        if self.entity.is_empty() {
            return Err(wrong_query("entity needed"));
        }
        if self.customer.models_related.contains(self.entity.as_ref()) {
            return Ok(());
        }
        //TODO: clean the response message
        Err(wrong_query(&format!("wrong entity {:?}", self.entity)))
    }

    pub async fn save_embed_query(payload: &EmbedRecommendationRequest) -> u32 {
        let (fields, values) = payload.get_fields_and_values().await;
        match EmbedRecommendationRequestModel::create(&fields, &values).await {
            Ok(result) => result.id,
            Err(_) => 0,
        }
    }

    pub async fn save_api_query(payload: &APIRecommendationRequest) -> u32 {
        let (fields, values) = payload.get_fields_and_values().await;
        println!("{:?}", values);
        match APIRecommendationRequestModel::create(&fields, &values).await {
            Ok(result) => result.id,
            Err(_) => 0,
        }
    }
}
