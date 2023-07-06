use async_trait::async_trait;
use envy::get_env;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnection, SqliteRow};
use sqlx::FromRow;
use std::collections::HashMap;

#[derive(Debug)]
pub enum CRUDError {
    NotFound,
    MaxRetry,
    WrongParameters,
    Write,
    Delete,
}

#[async_trait]
pub trait Manager<'a>: Sized
where
    Self::Item: for<'r> FromRow<'r, SqliteRow>,
{
    type Item: Deserialize<'a> + Serialize + Send + Sync + Unpin;

    async fn get(id: u32) -> Result<Self::Item, CRUDError> {
        Self::execute_query(
            format!("SELECT * FROM {} WHERE id = {id}", Self::table().await),
            Self::connect().await,
        )
        .await
    }

    async fn find(query_param: &HashMap<String, String>) -> Result<Vec<Self::Item>, CRUDError> {
        let query_string = query_param
            .iter()
            .map(|(key, value)| format!("{} = {}", key, value))
            .collect::<Vec<_>>()
            .join(" AND ");

        let query = format!(
            "SELECT * FROM {} WHERE {};",
            Self::table().await,
            query_string
        );

        let result = sqlx::query_as::<_, Self::Item>(&query)
            .fetch_all(&mut Self::connect().await)
            .await;
        match result {
            Ok(rows) => Ok(rows),
            Err(_) => Err(CRUDError::NotFound),
        }
    }

    async fn create(parameters: &HashMap<String, String>) -> Result<Self::Item, CRUDError> {
        let (fields, placeholders): (Vec<_>, Vec<_>) = parameters
            .iter()
            .map(|(key, _)| (format!("\"{}\"", key), "?"))
            .unzip();

        let fields = fields.join(", ");
        let placeholders = placeholders.join(", ");

        Self::execute_query(
            format!(
                "INSERT INTO {} ({}) VALUES ({});",
                Self::table().await,
                fields,
                placeholders
            ),
            Self::connect().await,
        )
        .await
    }

    async fn update(
        id: u32,
        parameters: &HashMap<String, String>,
    ) -> Result<Self::Item, CRUDError> {
        let fields_names = parameters
            .iter()
            .map(|(key, value)| format!("{key} = {value}"))
            .collect::<Vec<_>>()
            .join(",");

        Self::execute_query(
            format!(
                "UPDATE {table} SET {fields_names} WHERE id = {id}",
                table = Self::table().await
            ),
            Self::connect().await,
        )
        .await
    }

    async fn delete(id: u32) -> Result<u64, CRUDError> {
        let mut conn = Self::connect().await;
        let query = format!(
            "DELETE FROM {table} WHERE id = {id}",
            table = Self::table().await
        );
        match sqlx::query(&query).execute(&mut conn).await {
            Ok(row) => Ok(row.rows_affected()),
            Err(_) => Err(CRUDError::NotFound),
        }
    }

    async fn connect() -> SqliteConnection {
        match SqliteConnection::connect(&get_env("DATABASE_URL")).await {
            Ok(db) => db,
            Err(e) => panic!("{}", e),
        }
    }

    async fn execute_query(
        query: String,
        mut conn: SqliteConnection,
    ) -> Result<Self::Item, CRUDError> {
        let row = sqlx::query_as::<_, Self::Item>(&query)
            .fetch_one(&mut conn)
            .await;
        match row {
            Ok(row) => Ok(row),
            Err(_) => Err(CRUDError::NotFound),
        }
    }

    async fn table() -> String;
}
