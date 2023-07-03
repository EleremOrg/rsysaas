use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendationQueryRequest {
    pub token: Arc<String>,
    pub user_id: u32,
    pub prod_id: u32,
    pub num_recs: u8,
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
