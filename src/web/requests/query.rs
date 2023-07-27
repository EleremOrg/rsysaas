use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::business::versioning::Version;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PathRequest {
    pub version: Version,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelQueryRequest {
    pub fields: Option<HashMap<String, String>>,
    pub limit: Option<u8>,
}

impl ModelQueryRequest {
    pub fn get_query(&self) -> HashMap<String, String> {
        let mut fields = self.get_fields().clone();
        fields.insert(
            String::from("limit"),
            self.limit.unwrap_or_else(|| 50).to_string(),
        );
        fields
    }
    pub fn get_fields_and_values(&self) -> (String, String) {
        let mut fields = String::from("");
        let mut values = String::from("");
        for (key, value) in &self.get_fields() {
            fields.push_str(format!("{fields}, {key}").as_str());
            values.push_str(format!("{values}, {value}").as_str());
        }
        (fields, values)
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
