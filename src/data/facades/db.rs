use crate::data::errors::CRUDError;
use axum::async_trait;
use envy::get_env;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{
    sqlite::{SqliteConnection, SqlitePool, SqliteRow},
    Connection, FromRow, Row,
};
use std::collections::HashMap;

#[async_trait]
pub trait Manager<'a>
where
    Self: for<'r> FromRow<'r, SqliteRow> + Deserialize<'a> + Serialize + Send + Sync + Unpin,
{
    async fn get(id: u32) -> Result<Self, CRUDError> {
        Self::execute_query(
            format!("SELECT * FROM {} WHERE id = {id}", Self::table().await),
            Self::connect().await,
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

        let rows = sqlx::query_as::<_, Self>(&query)
            .fetch_all(&mut Self::connect().await)
            .await;

        match rows {
            Ok(json) => Ok(json),
            Err(_e) => Err(CRUDError::WrongParameters),
        }
    }

    async fn create(fields: &str, values: &str) -> Result<Self, CRUDError> {
        //TODO: make it more beautiful
        let mut transaction = match Self::transaction().await.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                println!("transaction errror launching: {:?}", err);
                return Err(CRUDError::NotFound);
            }
        };
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
                println!("run insert: {:?}", err);
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
                    Ok(_) => println!("transacttion commit succeeded"),
                    Err(err) => {
                        println!("transacttion commit error: {:?}", err);
                        return Err(CRUDError::NotFound);
                    }
                };
                Ok(row)
            }
            Err(err) => {
                println!("run fetch: {:?}", err);
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
            Err(_err) => Err(CRUDError::NotFound),
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
        //TODO: use a pool
        match SqliteConnection::connect(&get_env("DATABASE_URL")).await {
            Ok(db) => db,
            Err(e) => panic!("{}", e),
        }
    }

    async fn transaction() -> SqlitePool {
        match SqlitePool::connect(&get_env("DATABASE_URL")).await {
            Ok(db) => db,
            Err(e) => panic!("{}", e),
        }
    }

    async fn execute_query(query: String, mut conn: SqliteConnection) -> Result<Self, CRUDError> {
        let row = sqlx::query_as::<_, Self>(&query).fetch_one(&mut conn).await;
        match row {
            Ok(row) => Ok(row),
            Err(err) => {
                println!("{:?}", err);
                Err(CRUDError::NotFound)
            }
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
