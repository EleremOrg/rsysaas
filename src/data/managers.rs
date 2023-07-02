use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug)]
pub enum CRUDError {
    NotFound,
    MaxRetry,
    Write,
    Delete,
}

pub trait Manager {
    type Item: DeserializeOwned + Serialize;

    fn get(id: u32) -> Result<Self::Item, CRUDError>;
    fn find();
    fn create();
    fn update();
    fn delete();
}
