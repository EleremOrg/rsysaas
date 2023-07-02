use super::managers::CRUDError;
use redis::{Client, Commands, Connection, RedisError};
use serde::{de::DeserializeOwned, Serialize};

pub trait RedisManager {
    type Item: DeserializeOwned + Serialize;

    fn key(key: u32) -> String {
        format!("{:?}{key}", Self::prefix())
    }

    fn prefix() -> String;

    fn get<T>(key: u32) -> Result<Self::Item, CRUDError> {
        let mut conn = Self::connect();
        let result: Result<String, _> = conn.hgetall(Self::key(key));
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
        Err(CRUDError::MaxRetry)
    }

    fn set(key: u32, value: &Self::Item) -> Result<&Self::Item, CRUDError> {
        let mut conn = Self::connect();
        match serde_json::to_string(value) {
            Ok(json_value) => {
                let result: Result<u32, RedisError> = conn.set(Self::key(key), json_value); // use hset_multiple
                match result {
                    Ok(_) => Ok(&value),
                    Err(err) => {
                        eprintln!("Failed to fetch item data: {:?}", err);
                        return Err(CRUDError::Write);
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to fetch item data: {:?}", err);
                return Err(CRUDError::Write);
            }
        }
    }
    fn delete(key: u32) -> Result<(String, u32), CRUDError> {
        let mut conn = Self::connect();
        let redis_key = Self::key(key);
        let result: Result<u32, RedisError> = conn.del(redis_key.clone());
        match result {
            Ok(_) => Ok((redis_key, key)),
            Err(err) => {
                eprintln!("Failed to delete item: {:?}", err);
                return Err(CRUDError::Delete);
            }
        }
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
