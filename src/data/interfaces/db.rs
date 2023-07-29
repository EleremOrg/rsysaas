use std::collections::HashMap;

use axum::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{
    sqlite::{SqliteConnection, SqlitePool, SqliteRow},
    Connection, FromRow, Row, Sqlite, Transaction,
};
use tracing::error;

use envy::get_env;

use crate::data::errors::CRUDError;

#[async_trait]
pub trait Manager<'a>
where
    Self: for<'r> FromRow<'r, SqliteRow> + Deserialize<'a> + Serialize + Send + Sync + Unpin,
{
    async fn get(id: u32) -> Result<Self, CRUDError> {
        Self::execute_query(
            format!("SELECT * FROM {} WHERE id = {id}", Self::table().await),
            Self::transaction().await?,
        )
        .await
    }

    async fn get_all() -> Result<Vec<Self>, CRUDError> {
        Self::rows_to_vec(
            format!("SELECT * FROM {}", Self::table().await),
            Self::transaction().await?,
        )
        .await
    }

    // TODO: fix the query
    async fn find(&self, mut query_param: HashMap<String, String>) -> Result<Vec<Self>, CRUDError> {
        let limit = query_param.remove("limit").unwrap();
        let query_string = query_param
            .iter()
            .map(|(key, value)| format!("{} = {}", key, value))
            .collect::<Vec<_>>()
            .join(" AND ");

        let query = format!(
            "SELECT * FROM {} WHERE {} LIMIT {};",
            Self::table().await,
            query_string,
            limit
        );

        Self::rows_to_vec(query, Self::transaction().await?).await
    }

    async fn create(fields: &str, values: &str) -> Result<Self, CRUDError> {
        //TODO: make it more beautiful
        let mut transaction = Self::transaction().await?;
        let query = format!(
            "INSERT INTO {table} ({fields}) VALUES ({values});",
            table = Self::table().await
        );

        match sqlx::query(&query)
            .execute(&mut transaction as &mut SqliteConnection)
            .await
        {
            Ok(row) => row,
            Err(err) => {
                error!("run insert: {:?}", err);
                return Err(CRUDError::NotFound);
            }
        };

        let retreival_query = format!(
            "SELECT * FROM {} WHERE id = last_insert_rowid()",
            Self::table().await
        );
        // Fetch the inserted row from the database
        match sqlx::query_as::<_, Self>(&retreival_query)
            .fetch_one(&mut transaction as &mut SqliteConnection)
            .await
        {
            Ok(row) => {
                match transaction.commit().await {
                    Ok(_) => error!("transacttion commit succeeded"),
                    Err(err) => {
                        error!("transacttion commit error: {:?}", err);
                        return Err(CRUDError::NotFound);
                    }
                };
                Ok(row)
            }
            Err(err) => {
                error!("run fetch: {:?}", err);
                Err(CRUDError::NotFound)
            }
        }
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
            Self::transaction().await?,
        )
        .await
    }

    async fn delete(&self, id: u32) -> Result<u64, CRUDError> {
        let query = format!(
            "DELETE FROM {table} WHERE id = {id}",
            table = Self::table().await
        );
        match sqlx::query(&query)
            .execute(&mut Self::transaction().await? as &mut SqliteConnection)
            .await
        {
            Ok(row) => Ok(row.rows_affected()),
            Err(err) => {
                error!("deleting: {:?}", err);
                Err(CRUDError::NotFound)
            }
        }
    }

    async fn exists(conditions: &str) -> Result<bool, CRUDError> {
        let query = format!(
            "SELECT EXISTS (SELECT 1 FROM {} WHERE {}) AS result;",
            Self::table().await,
            conditions
        );
        let query_result = sqlx::query(&query)
            .fetch_one(&mut Self::transaction().await? as &mut SqliteConnection)
            .await;
        let row = match query_result {
            Ok(row) => row,
            Err(err) => {
                error!("gettings row: {:?}", err);
                return Err(CRUDError::NotFound);
            }
        };
        match row.try_get("result") {
            Ok(result) => Ok(result),
            Err(err) => {
                error!("checking exists: {:?}", err);
                Err(CRUDError::InternalError)
            }
        }
    }

    async fn get_for_encoding(id: u32) -> Result<(String, Vec<String>), CRUDError> {
        let mut transaction = Self::transaction().await?;

        let all_query = format!("SELECT name FROM {}", Self::table().await);
        let rows = sqlx::query(&all_query)
            .fetch_all(&mut transaction as &mut SqliteConnection)
            .await;

        let references = match rows {
            Ok(json) => json,
            Err(err) => {
                error!("error findig: {:?}", err);
                return Err(CRUDError::WrongParameters);
            }
        };

        let query_one = format!("SELECT name FROM {} WHERE id = {id}", Self::table().await);
        let row = sqlx::query(&query_one)
            .fetch_one(&mut transaction as &mut SqliteConnection)
            .await;
        let main = match row {
            Ok(row) => row,
            Err(err) => {
                error!("executing query {:?}", err);
                return Err(CRUDError::NotFound);
            }
        };
        Ok((String::from(""), vec![]))
    }

    async fn execute_query(
        query: String,
        mut transaction: Transaction<'a, Sqlite>,
    ) -> Result<Self, CRUDError> {
        let row = sqlx::query_as::<_, Self>(&query)
            .fetch_one(&mut transaction as &mut SqliteConnection)
            .await;
        match row {
            Ok(row) => Ok(row),
            Err(err) => {
                error!("executing query {:?}", err);
                Err(CRUDError::NotFound)
            }
        }
    }

    async fn rows_to_vec(
        query: String,
        mut transaction: Transaction<'a, Sqlite>,
    ) -> Result<Vec<Self>, CRUDError> {
        let rows = sqlx::query_as::<_, Self>(&query)
            .fetch_all(&mut transaction as &mut SqliteConnection)
            .await;

        match rows {
            Ok(json) => Ok(json),
            Err(err) => {
                error!("error findig: {:?}", err);
                return Err(CRUDError::WrongParameters);
            }
        }
    }

    async fn transaction() -> Result<Transaction<'a, Sqlite>, CRUDError> {
        match Self::connect().await.begin().await {
            Ok(transaction) => Ok(transaction),
            Err(err) => {
                error!("transaction errror launching: {:?}", err);
                return Err(CRUDError::InternalError);
            }
        }
    }

    async fn connect() -> SqlitePool {
        match SqlitePool::connect(&get_env("DATABASE_URL")).await {
            Ok(db) => db,
            Err(e) => panic!("{}", e),
        }
    }

    fn to_json(&self, result: Self) -> Result<Value, CRUDError> {
        match serde_json::to_value(&result) {
            Ok(value) => Ok(value),
            Err(err) => {
                error!("to json error {:?}", err);
                Err(CRUDError::JsonError)
            }
        }
    }

    async fn table() -> String;
}
