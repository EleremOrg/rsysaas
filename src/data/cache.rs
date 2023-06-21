use redis::{Client, Commands, Connection};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug)]
pub enum CRUDError {
    NotFound,
    MaxRetryError,
}

pub trait RedisManager {
    type Item: DeserializeOwned + Serialize;

    fn key(key: u32) -> String {
        format!("{:?}{key}", Self::prefix())
    }

    fn prefix() -> String;

    fn get(key: u32) -> Result<Self::Item, CRUDError> {
        let mut conn = Self::connect();
        let result: Result<String, _> = conn.get(Self::key(key));
        match result {
            Ok(data) => {
                let item: Result<Self::Item, _> = serde_json::from_str(&data);
                match item {
                    Ok(item) => Ok(item),
                    Err(err) => {
                        eprintln!("Failed to deserialize item data: {:?}", err);
                        // probably try again
                        Self::handle_deserialization_failed()
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to fetch item data: {:?}", err);
                Self::handle_not_found()
            }
        }
    }

    fn handle_not_found() -> Result<Self::Item, CRUDError> {
        Err(CRUDError::NotFound)
    }

    fn handle_deserialization_failed() -> Result<Self::Item, CRUDError> {
        Err(CRUDError::MaxRetryError)
    }

    fn set(key: u32, value: &Self::Item) {
        let mut conn = Self::connect();
        let json_value = serde_json::to_string(value).expect("Failed to serialize item as JSON");
        let _: () = conn
            .set(Self::key(key), json_value)
            .expect("Failed to set item in Redis");
    }

    fn delete(key: u32) {
        let mut conn = Self::connect();
        let _: () = conn
            .del(Self::key(key))
            .expect("Failed to delete item from Redis");
    }

    fn connect() -> Connection {
        match Client::open("redis://localhost:6379/") {
            Ok(client) => match client.get_connection() {
                Ok(conn) => conn,
                Err(err) => panic!("connection failed {:?}", err),
            },
            Err(err) => panic!("client failed {:?}", err),
        }
    }
}
