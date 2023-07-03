use std::collections::HashMap;

use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug)]
pub enum CRUDError {
    NotFound,
    MaxRetry,
    WrongParameters,
    Write,
    Delete,
}

pub trait Manager {
    type Item: DeserializeOwned + Serialize + Send + Sync;

    fn get(id: u32) -> Result<Self::Item, CRUDError>;
    fn find(query: &HashMap<String, String>) -> Result<Vec<Self::Item>, CRUDError>;
    fn create(query: &HashMap<String, String>) -> Result<Self::Item, CRUDError>;
    fn update(id: u32, query: &HashMap<String, String>) -> Result<Self::Item, CRUDError>;
    fn delete(id: u32) -> Result<u32, CRUDError>;
}
