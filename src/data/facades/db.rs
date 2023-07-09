use crate::data::errors::CRUDError;
use axum::async_trait;
use envy::get_env;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{
    sqlite::{SqliteConnection, SqliteRow},
    Connection, FromRow, Row,
};
use std::collections::HashMap;

#[async_trait]
pub trait Manager<'a>
where
    Self: for<'r> FromRow<'r, SqliteRow> + Deserialize<'a> + Serialize + Send + Sync + Unpin,
{
    async fn get(&self, id: u32) -> Result<Self, CRUDError> {
        Self::execute_query(
            format!("SELECT * FROM {} WHERE id = {id}", Self::table().await),
            Self::connect().await,
        )
        .await
    }

    // TODO: fix the query
    async fn find(&self, query_param: &HashMap<String, String>) -> Result<Vec<Self>, CRUDError> {
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

        let rows = sqlx::query_as::<_, Self>(&query)
            .fetch_all(&mut Self::connect().await)
            .await;

        match rows {
            Ok(json) => Ok(json),
            Err(_e) => Err(CRUDError::WrongParameters),
        }
    }

    async fn create(&self, parameters: &HashMap<String, String>) -> Result<Self, CRUDError> {
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
        &self,
        id: u32,
        parameters: &HashMap<String, String>,
    ) -> Result<Self, CRUDError> {
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

    async fn delete(&self, id: u32) -> Result<u64, CRUDError> {
        let query = format!(
            "DELETE FROM {table} WHERE id = {id}",
            table = Self::table().await
        );
        match sqlx::query(&query)
            .execute(&mut Self::connect().await)
            .await
        {
            Ok(row) => Ok(row.rows_affected()),
            Err(_) => Err(CRUDError::NotFound),
        }
    }

    async fn exists(conditions: &str) -> Result<bool, CRUDError> {
        let query = format!(
            "SELECT EXISTS (SELECT 1 FROM {} WHERE {}) AS result;",
            Self::table().await,
            conditions
        );
        let query_result = sqlx::query(&query)
            .fetch_one(&mut Self::connect().await)
            .await;
        let row = match query_result {
            Ok(row) => row,
            Err(_err) => return Err(CRUDError::NotFound),
        };
        match row.try_get("result") {
            Ok(result) => Ok(result),
            Err(_) => Err(CRUDError::InternalError),
        }
    }

    async fn connect() -> SqliteConnection {
        match SqliteConnection::connect(&get_env("DATABASE_URL")).await {
            Ok(db) => db,
            Err(e) => panic!("{}", e),
        }
    }

    async fn execute_query(query: String, mut conn: SqliteConnection) -> Result<Self, CRUDError> {
        let row = sqlx::query_as::<_, Self>(&query).fetch_one(&mut conn).await;
        match row {
            Ok(row) => Ok(row),
            Err(_) => Err(CRUDError::NotFound),
        }
    }

    fn to_json(&self, result: Self) -> Result<Value, CRUDError> {
        match serde_json::to_value(&result) {
            Ok(value) => Ok(value),
            Err(_e) => Err(CRUDError::JsonError),
        }
    }

    async fn table() -> String;
}
