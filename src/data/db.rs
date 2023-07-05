use crate::utils::get_env;
use axum::async_trait;
use sqlx::sqlite::SqliteConnection;
use sqlx::{Connection};

#[async_trait]
pub trait DBManager {
    type Item: serde::de::DeserializeOwned + serde::Serialize;

    async fn connect() -> SqliteConnection {
        match SqliteConnection::connect(&get_env("DATABASE_URL")).await {
            Ok(db) => db,
            Err(e) => panic!("{}", e),
        }
    }
}
