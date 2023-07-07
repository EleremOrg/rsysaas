use crate::errors::CRUDError;
use async_trait::async_trait;
use envy::get_env;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{
    sqlite::{SqliteConnection, SqliteRow},
    Connection, FromRow,
};
use std::collections::HashMap;

#[async_trait]
pub trait Manager<'a>: Sized
where
    Self: for<'r> FromRow<'r, SqliteRow> + Deserialize<'a> + Serialize + Send + Sync + Unpin,
{
    async fn get(&self, id: u32) -> Result<Self, CRUDError> {
        self.execute_query(
            format!("SELECT * FROM {} WHERE id = {id}", Self::table().await),
            self.connect().await,
        )
        .await
    }

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
            .fetch_all(&mut self.connect().await)
            .await;

        match rows {
            Ok(json) => Ok(json),
            Err(e) => Err(CRUDError::WrongParameters),
        }
    }

    async fn create(&self, parameters: &HashMap<String, String>) -> Result<Self, CRUDError> {
        let (fields, placeholders): (Vec<_>, Vec<_>) = parameters
            .iter()
            .map(|(key, _)| (format!("\"{}\"", key), "?"))
            .unzip();

        let fields = fields.join(", ");
        let placeholders = placeholders.join(", ");

        self.execute_query(
            format!(
                "INSERT INTO {} ({}) VALUES ({});",
                Self::table().await,
                fields,
                placeholders
            ),
            self.connect().await,
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

        self.execute_query(
            format!(
                "UPDATE {table} SET {fields_names} WHERE id = {id}",
                table = Self::table().await
            ),
            self.connect().await,
        )
        .await
    }

    async fn delete(&self, id: u32) -> Result<u64, CRUDError> {
        let mut conn = self.connect().await;
        let query = format!(
            "DELETE FROM {table} WHERE id = {id}",
            table = Self::table().await
        );
        match sqlx::query(&query).execute(&mut conn).await {
            Ok(row) => Ok(row.rows_affected()),
            Err(_) => Err(CRUDError::NotFound),
        }
    }

    async fn connect(&self) -> SqliteConnection {
        match SqliteConnection::connect(&get_env("DATABASE_URL")).await {
            Ok(db) => db,
            Err(e) => panic!("{}", e),
        }
    }

    async fn execute_query(
        &self,
        query: String,
        mut conn: SqliteConnection,
    ) -> Result<Self, CRUDError> {
        let row = sqlx::query_as::<_, Self>(&query).fetch_one(&mut conn).await;
        match row {
            Ok(row) => Ok(row),
            Err(_) => Err(CRUDError::NotFound),
        }
    }

    fn to_json(&self, result: Self) -> Result<Value, CRUDError> {
        match serde_json::to_value(&result) {
            Ok(value) => Ok(value),
            Err(e) => Err(CRUDError::JsonError),
        }
    }

    async fn table() -> String;
}
