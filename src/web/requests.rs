use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

use crate::business::versioning::Version;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIRecommendationQueryRequest {
    pub entity: Arc<String>,
    pub user_id: Option<u32>,
    pub prod_id: Option<u32>,
    pub number_recommendations: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PathRequest {
    pub version: Version,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryRequest {
    pub fields: Option<HashMap<String, String>>,
    pub limit: Option<u8>,
}

impl QueryRequest {
    pub fn get_query(&self) -> HashMap<String, String> {
        let mut fields = self.get_fields().clone();
        fields.insert(
            String::from("limit"),
            self.limit.unwrap_or_else(|| 50).to_string(),
        );
        fields
    }
    pub fn get_params(&self) -> HashMap<String, String> {
        self.get_fields()
    }
    fn get_fields(&self) -> HashMap<String, String> {
        match self.fields.clone() {
            Some(fields) => fields,
            None => HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmbedRecommendationQueryRequest {
    pub orientation: Arc<String>,
    pub entity: Arc<String>,
    pub title: Arc<String>,
    pub show_image: bool,
    pub show_resume: bool,
    pub user_id: Option<u32>,
    pub prod_id: Option<u32>,
    pub number_recommendations: Option<u8>,
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
