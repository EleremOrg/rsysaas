use async_trait::async_trait;
use envy::get_env;
use futures::stream::{FuturesUnordered, StreamExt};
use sqlx::sqlite::{SqliteConnection, SqliteRow};
use sqlx::Connection;
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
pub trait DBManager: Sized {
    const TABLE: String;

    async fn get(id: u32) -> Result<Self, CRUDError> {
        let mut conn = Self::connect().await;
        let query = format!("SELECT * FROM {} WHERE id = ?", Self::table().await);
        let row = sqlx::query(&query).bind(id).fetch_one(&mut conn).await;
        match row {
            Ok(row) => Ok(Self::row_to_item(&row).await),
            Err(_) => Err(CRUDError::NotFound),
        }
    }

    async fn row_to_item(row: &SqliteRow) -> Self;

    async fn find(query_param: &HashMap<String, String>) -> Result<Vec<Self>, CRUDError> {
        let mut conn = Self::connect().await;

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
        let mut result: Vec<Self> = Vec::new();

        match sqlx::query(&query).fetch_all(&mut conn).await {
            Ok(rows) => Ok(Self::rows_to_items(rows).await),
            Err(_) => Err(CRUDError::NotFound),
        }
    }

    async fn rows_to_items(rows: Vec<SqliteRow>) -> Vec<Self> {
        let mut futures = FuturesUnordered::new();
        rows.iter()
            .for_each(|row| futures.push(Self::row_to_item(&row)));

        futures.collect::<Vec<Self>>().await
    }

    async fn create(parameters: &HashMap<String, String>) -> Result<Self, CRUDError> {
        let mut conn = Self::connect().await;
        let table = Self::table().await;

        let (fields, placeholders): (Vec<_>, Vec<_>) = parameters
            .iter()
            .map(|(key, _)| (format!("\"{}\"", key), "?"))
            .unzip();

        let fields = fields.join(", ");
        let placeholders = placeholders.join(", ");

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({});",
            table, fields, placeholders
        );

        match sqlx::query(&query).fetch_one(&mut conn).await {
            Ok(row) => Ok(Self::row_to_item(&row).await),
            Err(_) => Err(CRUDError::Write),
        }
    }

    async fn update(id: u32, parameters: &HashMap<String, String>) -> Result<Self, CRUDError> {
        let mut conn = Self::connect().await;
        let fields_names = parameters
            .iter()
            .map(|(key, value)| format!("{key} = {value}"))
            .collect::<Vec<_>>()
            .join(",");
        let query = format!(
            "UPDATE {table} SET {fields_names} WHERE id = {id}",
            table = Self::table().await
        );

        match sqlx::query(&query).fetch_one(&mut conn).await {
            Ok(row) => Ok(Self::row_to_item(&row).await),
            Err(_) => Err(CRUDError::Write),
        }
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

    async fn table() -> String {
        Self::TABLE
    }
}
