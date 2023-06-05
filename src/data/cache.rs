use redis::{Client, Commands, Connection};
use serde::{de::DeserializeOwned, Serialize};

pub trait RedisManager {
    type Item: DeserializeOwned + Serialize;

    fn key(key: u32) -> String {
        format!("{:?}{key}", Self::prefix())
    }

    fn prefix() -> String;

    fn get(key: u32) -> Option<Self::Item> {
        let conn = Self::connect();
        let result: Result<String, _> = conn.get(Self::key(key));
        match result {
            Ok(data) => {
                let item: Result<Self::Item, _> = serde_json::from_str(&data);
                match item {
                    Ok(item) => Some(item),
                    Err(err) => {
                        eprintln!("Failed to deserialize item data: {:?}", err);
                        None
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to fetch item data: {:?}", err);
                None
            }
        }
    }

    fn set(key: u32, value: &Self::Item) {
        let conn = Self::connect();
        let json_value = serde_json::to_string(value).expect("Failed to serialize item as JSON");
        let _: () = conn
            .set(Self::key(key), json_value)
            .expect("Failed to set item in Redis");
    }

    fn delete(key: u32) {
        let conn = Self::connect();
        let _: () = conn
            .del(Self::key(key))
            .expect("Failed to delete item from Redis");
    }

    fn connect() -> Connection {
        match Client::open("redis://127.0.0.1/") {
            Ok(client) => match client.get_connection() {
                Ok(conn) => conn,
                Err(err) => panic!("connection failed {:?}", err),
            },
            Err(err) => panic!("client failed {:?}", err),
        }
    }
}
